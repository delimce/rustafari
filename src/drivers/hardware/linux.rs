use super::traits::{constants::UNKNOWN_VALUE, HardwareProvider};
use battery::{
    units::{energy::watt_hour, ratio::percent, Energy, Ratio},
    Manager,
};
use local_ip_address::local_ip;
use std::fs;
use std::process::Command;
use sysinfo::{CpuExt, CpuRefreshKind, DiskExt, RefreshKind, System, SystemExt};

pub struct LinuxHardware;

impl HardwareProvider for LinuxHardware {
    fn cpu_model() -> String {
        Self::get_system().cpus()[0].brand().to_string()
    }

    fn cpu_cores() -> u32 {
        sys_info::cpu_num().unwrap_or(0)
    }

    fn hostname() -> String {
        sys_info::hostname().unwrap_or_else(|_| UNKNOWN_VALUE.to_string())
    }

    fn mac_address() -> String {
        mac_address::get_mac_address()
            .ok()
            .flatten()
            .map(|addr| addr.to_string())
            .unwrap_or_else(|| UNKNOWN_VALUE.to_string())
    }

    fn cpu_cache() -> (u64, u64, u64) {
        (
            cache_size::l1_cache_size().unwrap_or(0) as u64,
            cache_size::l2_cache_size().unwrap_or(0) as u64,
            cache_size::l3_cache_size().unwrap_or(0) as u64,
        )
    }

    fn local_ip() -> String {
        local_ip()
            .map(|ip| ip.to_string())
            .unwrap_or_else(|_| UNKNOWN_VALUE.to_string())
    }

    fn device_serial() -> String {
        // Try various methods to get device serial on Linux
        if let Some(serial) = Self::get_serial_from_dmi() {
            return serial;
        }

        if let Some(serial) = Self::get_serial_from_proc() {
            return serial;
        }

        // Last resort: use mid crate
        match mid::data("mySecretKey") {
            Ok(serials) => serials
                .result
                .get(1)
                .cloned()
                .unwrap_or_else(|| UNKNOWN_VALUE.to_string()),
            Err(_) => UNKNOWN_VALUE.to_string(),
        }
    }

    fn has_battery() -> bool {
        Self::get_battery_manager()
            .map(|manager| {
                manager
                    .batteries()
                    .map(|mut batteries| batteries.any(|b| b.is_ok()))
                    .unwrap_or(false)
            })
            .unwrap_or(false)
    }

    fn memory_info() -> (u64, u64) {
        sys_info::mem_info()
            .map(|mem| (mem.total, mem.free))
            .unwrap_or((0, 0))
    }

    fn disk_info() -> (u64, u64) {
        let mut system = System::new_all();
        system.refresh_all();

        system
            .disks()
            .first()
            .map(|disk| (disk.total_space(), disk.available_space()))
            .unwrap_or((0, 0))
    }

    fn battery_info() -> (Ratio, Energy) {
        if let Ok(manager) = Self::get_battery_manager() {
            if let Ok(batteries) = manager.batteries() {
                for battery in batteries {
                    if let Ok(battery) = battery {
                        return (battery.state_of_health(), battery.energy_full());
                    }
                }
            }
        }
        (Ratio::new::<percent>(0.0), Energy::new::<watt_hour>(0.0))
    }

    fn external_ip() -> String {
        reqwest::blocking::Client::new()
            .get("https://api.ipify.org")
            .send()
            .and_then(|response| response.text())
            .unwrap_or_else(|_| UNKNOWN_VALUE.to_string())
    }

    fn manufactured_date() -> String {
        Self::get_manufactured_date().unwrap_or_else(|| UNKNOWN_VALUE.to_string())
    }

    fn system_manufacturer() -> String {
        Self::get_system_manufacturer().unwrap_or_else(|| UNKNOWN_VALUE.to_string())
    }

    fn system_product_name() -> String {
        Self::get_system_product_name().unwrap_or_else(|| UNKNOWN_VALUE.to_string())
    }
}

impl LinuxHardware {
    /// Create a system instance with CPU information
    fn get_system() -> System {
        System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()))
    }

    /// Create a battery manager instance
    fn get_battery_manager() -> Result<Manager, battery::Error> {
        Manager::new()
    }

    /// Get device serial number from DMI information
    fn get_serial_from_dmi() -> Option<String> {
        let dmi_paths = [
            "/sys/class/dmi/id/product_serial",
            "/sys/class/dmi/id/board_serial",
            "/sys/class/dmi/id/chassis_serial",
        ];

        for path in &dmi_paths {
            if let Some(serial) = Self::read_dmi_file(path) {
                if !serial.is_empty()
                    && serial != "Not Specified"
                    && serial != "To be filled by O.E.M."
                {
                    return Some(serial);
                }
            }
        }
        None
    }

    /// Get device serial number from /proc/cpuinfo (fallback)
    fn get_serial_from_proc() -> Option<String> {
        fs::read_to_string("/proc/cpuinfo")
            .ok()
            .and_then(|content| {
                for line in content.lines() {
                    if line.starts_with("Serial") {
                        if let Some(serial) = line.split(':').nth(1) {
                            let serial = serial.trim();
                            if !serial.is_empty() && serial != "0000000000000000" {
                                return Some(serial.to_string());
                            }
                        }
                    }
                }
                None
            })
    }

    /// Get manufacturing date from various Linux sources
    fn get_manufactured_date() -> Option<String> {
        // Try BIOS date first
        if let Some(date) = Self::read_dmi_file("/sys/class/dmi/id/bios_date") {
            if !date.is_empty() && date != "Not Specified" {
                return Some(Self::format_bios_date(&date));
            }
        }

        // Try chassis/board serial which sometimes contains date info
        let serial_paths = [
            "/sys/class/dmi/id/chassis_serial",
            "/sys/class/dmi/id/board_serial",
        ];

        for path in &serial_paths {
            if let Some(serial) = Self::read_dmi_file(path) {
                if let Some(date) = Self::extract_date_from_serial(&serial) {
                    return Some(date);
                }
            }
        }

        // Try dmidecode as last resort (requires root)
        Self::get_date_from_dmidecode()
    }

    /// Format BIOS date to more readable format
    fn format_bios_date(bios_date: &str) -> String {
        // BIOS dates are usually in MM/DD/YYYY or DD/MM/YYYY format
        if let Some(formatted) = Self::parse_date_format(bios_date) {
            return formatted;
        }
        bios_date.to_string()
    }

    /// Try to parse and format date strings
    fn parse_date_format(date_str: &str) -> Option<String> {
        // Handle MM/DD/YYYY format
        let parts: Vec<&str> = date_str.split('/').collect();
        if parts.len() == 3 {
            if let (Ok(month), Ok(_day), Ok(year)) = (
                parts[0].parse::<u32>(),
                parts[1].parse::<u32>(),
                parts[2].parse::<u32>(),
            ) {
                if year >= 1000 && month >= 1 && month <= 12 {
                    let month_name = match month {
                        1 => "Jan",
                        2 => "Feb",
                        3 => "Mar",
                        4 => "Apr",
                        5 => "May",
                        6 => "Jun",
                        7 => "Jul",
                        8 => "Aug",
                        9 => "Sep",
                        10 => "Oct",
                        11 => "Nov",
                        12 => "Dec",
                        _ => return None,
                    };

                    return Some(format!("{} {}", month_name, year));
                }
            }
        }

        None
    }

    /// Try to extract manufacturing date from serial numbers
    fn extract_date_from_serial(serial: &str) -> Option<String> {
        // Some manufacturers encode date in serial numbers
        // This is a simplified implementation - real-world would need
        // manufacturer-specific decoding

        // Look for 4-digit year patterns (20XX)
        for i in 0..serial.len().saturating_sub(3) {
            let substring = &serial[i..i + 4];
            if let Ok(year) = substring.parse::<u32>() {
                if year >= 2000 && year <= 2030 {
                    return Some(year.to_string());
                }
            }
        }

        // Look for 2-digit year patterns (1X for 201X)
        for i in 0..serial.len().saturating_sub(1) {
            let substring = &serial[i..i + 2];
            if let Ok(year_suffix) = substring.parse::<u32>() {
                if year_suffix >= 10 && year_suffix <= 30 {
                    return Some(format!("{}", 2000 + year_suffix));
                }
            }
        }

        None
    }

    /// Try to get manufacturing date using dmidecode (requires root)
    fn get_date_from_dmidecode() -> Option<String> {
        Command::new("dmidecode")
            .args(&["-t", "system"])
            .output()
            .ok()
            .and_then(|output| {
                let output_str = String::from_utf8_lossy(&output.stdout);
                for line in output_str.lines() {
                    let line = line.trim();
                    if line.starts_with("Serial Number:") || line.starts_with("Version:") {
                        if let Some(value) = line.split(':').nth(1) {
                            let value = value.trim();
                            if let Some(date) = Self::extract_date_from_serial(value) {
                                return Some(date);
                            }
                        }
                    }
                }
                None
            })
    }

    /// Get system manufacturer from DMI
    fn get_system_manufacturer() -> Option<String> {
        Self::read_dmi_file("/sys/class/dmi/id/sys_vendor")
            .filter(|s| !s.is_empty() && s != "To be filled by O.E.M.")
    }

    /// Get system product name from DMI
    fn get_system_product_name() -> Option<String> {
        Self::read_dmi_file("/sys/class/dmi/id/product_name")
            .filter(|s| !s.is_empty() && s != "To be filled by O.E.M.")
    }

    /// Read and clean DMI file content
    fn read_dmi_file(path: &str) -> Option<String> {
        fs::read_to_string(path)
            .ok()
            .map(|content| content.trim().to_string())
            .filter(|content| !content.is_empty())
    }
}
