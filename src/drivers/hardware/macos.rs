use super::traits::{constants::UNKNOWN_VALUE, HardwareProvider};
use battery::{
    units::{energy::watt_hour, ratio::percent, Energy, Ratio},
    Manager,
};
use local_ip_address::local_ip;
use std::process::Command;
use sysinfo::{CpuExt, CpuRefreshKind, DiskExt, RefreshKind, System, SystemExt};

pub struct MacOSHardware;

impl HardwareProvider for MacOSHardware {
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
        // First try system_profiler
        if let Some(serial) = Self::get_serial_from_system_profiler() {
            return serial;
        }

        // Fallback to ioreg
        if let Some(serial) = Self::get_serial_from_ioreg() {
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
        "Apple Inc.".to_string()
    }

    fn system_product_name() -> String {
        Self::get_system_product_name().unwrap_or_else(|| UNKNOWN_VALUE.to_string())
    }
}

impl MacOSHardware {
    /// Create a system instance with CPU information
    fn get_system() -> System {
        System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()))
    }

    /// Create a battery manager instance
    fn get_battery_manager() -> Result<Manager, battery::Error> {
        Manager::new()
    }

    /// Get serial number from system_profiler command
    fn get_serial_from_system_profiler() -> Option<String> {
        Command::new("system_profiler")
            .args(&["SPHardwareDataType"])
            .output()
            .ok()
            .and_then(|output| {
                let output_str = String::from_utf8_lossy(&output.stdout);
                for line in output_str.lines() {
                    if line.trim().starts_with("Serial Number (system):") {
                        if let Some(serial) = line.split(':').nth(1) {
                            let serial = serial.trim();
                            if !serial.is_empty() && serial != "Not Available" {
                                return Some(serial.to_string());
                            }
                        }
                    }
                }
                None
            })
    }

    /// Get serial number from ioreg command
    fn get_serial_from_ioreg() -> Option<String> {
        Command::new("ioreg")
            .args(&["-c", "IOPlatformExpertDevice", "-d", "2"])
            .output()
            .ok()
            .and_then(|output| {
                let output_str = String::from_utf8_lossy(&output.stdout);
                for line in output_str.lines() {
                    if line.contains("IOPlatformSerialNumber") {
                        if let Some(start) = line.find("\"") {
                            if let Some(end) = line[start + 1..].find("\"") {
                                let serial = &line[start + 1..start + 1 + end];
                                if !serial.is_empty() {
                                    return Some(serial.to_string());
                                }
                            }
                        }
                    }
                }
                None
            })
    }

    /// Get manufacturing date from Apple serial number
    fn get_manufactured_date() -> Option<String> {
        // Get serial number and decode manufacturing date
        let serial =
            Self::get_serial_from_system_profiler().or_else(|| Self::get_serial_from_ioreg())?;

        Self::decode_apple_serial_date(&serial)
    }

    /// Decode manufacturing date from Apple serial number
    fn decode_apple_serial_date(serial: &str) -> Option<String> {
        if serial.len() != 10 && serial.len() != 12 {
            return None;
        }

        // Modern Apple serials: C07F10GTQ6NY
        // Position 3 (F) = year, Position 4 (1) = week in half-year
        let year_char = serial.chars().nth(3)?;
        let week_char = serial.chars().nth(4)?;

        // Apple's year encoding (position 3): cycles every few years
        let (base_year, half) = match year_char {
            'C' => (2010, 1),
            'D' => (2010, 2),
            'F' => (2011, 1),
            'G' => (2011, 2),
            'H' => (2012, 1),
            'J' => (2012, 2),
            'K' => (2013, 1),
            'L' => (2013, 2),
            'M' => (2014, 1),
            'N' => (2014, 2),
            'P' => (2015, 1),
            'Q' => (2015, 2),
            'R' => (2016, 1),
            'S' => (2016, 2),
            'T' => (2017, 1),
            'V' => (2017, 2),
            'W' => (2018, 1),
            'X' => (2018, 2),
            'Y' => (2019, 1),
            'Z' => (2019, 2),
            _ => return None,
        };

        // Check if this could be a newer cycle (after 2019, F would be 2020)
        let year = if year_char == 'F' && serial.starts_with("C07") {
            // Based on M1 Mac mini release (late 2020), this F is likely 2020
            2020
        } else {
            base_year
        };

        // Decode week from position 4
        let week_num = match week_char {
            '1'..='9' => week_char as u32 - '0' as u32,
            'A'..='Z' => 10 + (week_char as u32 - 'A' as u32),
            _ => return None,
        };

        // Calculate approximate month
        let month = if half == 1 {
            // First half of year
            match week_num {
                1..=4 => "Jan",
                5..=8 => "Feb",
                9..=13 => "Mar",
                14..=17 => "Apr",
                18..=22 => "May",
                23..=26 => "Jun",
                _ => "H1",
            }
        } else {
            // Second half of year
            match week_num {
                1..=4 => "Jul",
                5..=8 => "Aug",
                9..=13 => "Sep",
                14..=17 => "Oct",
                18..=22 => "Nov",
                23..=26 => "Dec",
                _ => "H2",
            }
        };

        Some(format!("{} {}", month, year))
    }

    /// Get system product name using system_profiler or sysctl
    fn get_system_product_name() -> Option<String> {
        // Try to get model from system_profiler
        if let Ok(output) = Command::new("system_profiler")
            .args(&["SPHardwareDataType"])
            .output()
        {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                if line.trim().starts_with("Model Name:") {
                    if let Some(model) = line.split(':').nth(1) {
                        return Some(model.trim().to_string());
                    }
                }
                if line.trim().starts_with("Model Identifier:") {
                    if let Some(model) = line.split(':').nth(1) {
                        return Some(model.trim().to_string());
                    }
                }
            }
        }

        // Fallback to sysctl
        Command::new("sysctl")
            .args(&["-n", "hw.model"])
            .output()
            .ok()
            .and_then(|output| {
                let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !result.is_empty() {
                    Some(result)
                } else {
                    None
                }
            })
    }
}
