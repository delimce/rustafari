use battery::{
    units::{energy::watt_hour, ratio::percent, Energy, Ratio},
    Manager,
};
use local_ip_address::local_ip;
use sysinfo::{CpuExt, CpuRefreshKind, DiskExt, RefreshKind, System, SystemExt};

const UNKNOWN_VALUE: &str = "Unknown";

pub struct Hardware;

impl Hardware {
    fn system() -> System {
        System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()))
    }

    fn battery_manager() -> Result<Manager, battery::Error> {
        Manager::new()
    }

    pub fn cpu_model() -> String {
        Self::system().cpus()[0].brand().to_string()
    }

    pub fn cpu_cores() -> u32 {
        sys_info::cpu_num().unwrap_or(0)
    }

    pub fn hostname() -> String {
        sys_info::hostname().unwrap_or_else(|_| UNKNOWN_VALUE.to_string())
    }

    pub fn mac_address() -> String {
        mac_address::get_mac_address()
            .ok()
            .flatten()
            .map(|addr| addr.to_string())
            .unwrap_or_else(|| UNKNOWN_VALUE.to_string())
    }

    pub fn cpu_cache() -> (u64, u64, u64) {
        (
            cache_size::l1_cache_size().unwrap_or(0) as u64,
            cache_size::l2_cache_size().unwrap_or(0) as u64,
            cache_size::l3_cache_size().unwrap_or(0) as u64,
        )
    }

    pub fn local_ip() -> String {
        local_ip()
            .map(|ip| ip.to_string())
            .unwrap_or_else(|_| UNKNOWN_VALUE.to_string())
    }

    pub fn device_serial() -> String {
        match mid::data("mySecretKey") {
            Ok(serials) => serials
                .result
                .get(1)
                .cloned()
                .unwrap_or_else(|| UNKNOWN_VALUE.to_string()),
            Err(_) => UNKNOWN_VALUE.to_string(),
        }
    }

    pub fn has_battery() -> bool {
        Self::battery_manager()
            .map(|manager| {
                manager
                    .batteries()
                    .map(|mut batteries| batteries.any(|b| b.is_ok()))
                    .unwrap_or(false)
            })
            .unwrap_or(false)
    }

    pub fn memory_info() -> (u64, u64) {
        sys_info::mem_info()
            .map(|mem| (mem.total, mem.free))
            .unwrap_or((0, 0))
    }

    pub fn disk_info() -> (u64, u64) {
        let mut system = System::new_all();
        system.refresh_all();

        system
            .disks()
            .first()
            .map(|disk| (disk.total_space(), disk.available_space()))
            .unwrap_or((0, 0))
    }

    pub fn battery_info() -> (Ratio, Energy) {
        if let Ok(manager) = Self::battery_manager() {
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

    pub fn external_ip() -> String {
        reqwest::blocking::Client::new()
            .get("https://api.ipify.org")
            .send()
            .and_then(|response| response.text())
            .unwrap_or_else(|_| UNKNOWN_VALUE.to_string())
    }

    pub fn manufactured_date() -> String {
        #[cfg(target_os = "macos")]
        {
            if let Some(date) = Self::get_manufactured_date_macos() {
                return date;
            }
        }

        #[cfg(target_os = "linux")]
        {
            if let Some(date) = Self::get_manufactured_date_linux() {
                return date;
            }
        }

        UNKNOWN_VALUE.to_string()
    }

    pub fn system_manufacturer() -> String {
        #[cfg(target_os = "macos")]
        {
            if let Some(manufacturer) = Self::get_system_manufacturer_macos() {
                return manufacturer;
            }
        }

        #[cfg(target_os = "linux")]
        {
            if let Some(manufacturer) = Self::get_system_manufacturer_linux() {
                return manufacturer;
            }
        }

        UNKNOWN_VALUE.to_string()
    }

    pub fn system_product_name() -> String {
        #[cfg(target_os = "macos")]
        {
            if let Some(product) = Self::get_system_product_name_macos() {
                return product;
            }
        }

        #[cfg(target_os = "linux")]
        {
            if let Some(product) = Self::get_system_product_name_linux() {
                return product;
            }
        }

        UNKNOWN_VALUE.to_string()
    }

    #[cfg(target_os = "macos")]
    fn get_manufactured_date_macos() -> Option<String> {
        // Try to get system info from system_profiler for Serial Number which sometimes contains date info

        use std::process::Command;
        if let Ok(output) = Command::new("system_profiler")
            .args(&["SPHardwareDataType"])
            .output()
        {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                if line.trim().starts_with("Serial Number (system):") {
                    if let Some(serial) = line.split(':').nth(1) {
                        let serial = serial.trim();
                        // Apple serials sometimes encode manufacturing info
                        if !serial.is_empty() && serial != "Not Available" {
                            return Some(format!("Serial: {}", serial));
                        }
                    }
                }
            }
        }

        // Try to get system version/build date
        if let Ok(output) = Command::new("sw_vers").args(&["-buildVersion"]).output() {
            let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !result.is_empty() {
                return Some(format!("Build: {}", result));
            }
        }

        // Fallback to system uptime info
        if let Ok(output) = Command::new("sysctl")
            .args(&["-n", "kern.boottime"])
            .output()
        {
            let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !result.is_empty() {
                return Some("System info available".to_string());
            }
        }

        None
    }

    #[cfg(target_os = "linux")]
    fn get_manufactured_date_linux() -> Option<String> {
        // Try various DMI paths on Linux
        let paths = [
            "/sys/class/dmi/id/bios_date",
            "/sys/class/dmi/id/chassis_serial", // Sometimes contains date info
            "/sys/class/dmi/id/board_serial",   // Sometimes contains date info
        ];

        for path in &paths {
            if let Some(content) = Self::read_dmi_file(path) {
                if !content.is_empty() && content != "Not Specified" {
                    return Some(content);
                }
            }
        }

        None
    }

    #[cfg(target_os = "macos")]
    fn get_system_manufacturer_macos() -> Option<String> {
        // For macOS, we know it's Apple
        Some("Apple Inc.".to_string())
    }

    #[cfg(target_os = "macos")]
    fn get_system_product_name_macos() -> Option<String> {
        // Try to get model from system_profiler

        use std::process::Command;
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
        if let Ok(output) = Command::new("sysctl").args(&["-n", "hw.model"]).output() {
            let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !result.is_empty() {
                return Some(result);
            }
        }

        None
    }

    #[cfg(target_os = "linux")]
    fn get_system_manufacturer_linux() -> Option<String> {
        Self::read_dmi_file("/sys/class/dmi/id/sys_vendor")
    }

    #[cfg(target_os = "linux")]
    fn get_system_product_name_linux() -> Option<String> {
        Self::read_dmi_file("/sys/class/dmi/id/product_name")
    }

    #[cfg(target_os = "linux")]
    fn read_dmi_file(path: &str) -> Option<String> {
        std::fs::read_to_string(path)
            .ok()
            .map(|content| content.trim().to_string())
            .filter(|content| !content.is_empty())
    }
}

// Public API functions
pub fn get_cpu_model() -> String {
    Hardware::cpu_model()
}
pub fn get_cpu_num() -> u32 {
    Hardware::cpu_cores()
}
pub fn get_mem_total() -> u64 {
    Hardware::memory_info().0
}
pub fn get_cpu_cache() -> (u64, u64, u64) {
    Hardware::cpu_cache()
}
pub fn get_disk_size() -> u64 {
    Hardware::disk_info().0
}
pub fn get_host_name() -> String {
    Hardware::hostname()
}
pub fn get_mac_address() -> String {
    Hardware::mac_address()
}
pub fn get_local_ip_address() -> String {
    Hardware::local_ip()
}
pub fn get_external_ip_address() -> String {
    Hardware::external_ip()
}
pub fn get_disk_free() -> u64 {
    Hardware::disk_info().1
}
pub fn get_device_serial() -> String {
    Hardware::device_serial()
}
pub fn get_battery_info() -> (Ratio, Energy) {
    Hardware::battery_info()
}
pub fn has_battery() -> bool {
    Hardware::has_battery()
}
pub fn _get_mem_free() -> u64 {
    Hardware::memory_info().1
}
pub fn get_manufactured_date() -> String {
    Hardware::manufactured_date()
}
pub fn get_system_manufacturer() -> String {
    Hardware::system_manufacturer()
}
pub fn get_system_product_name() -> String {
    Hardware::system_product_name()
}
