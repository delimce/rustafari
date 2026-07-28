use super::traits::constants::UNKNOWN_VALUE;
use battery::{
    units::{energy::watt_hour, ratio::percent, Energy, Ratio},
    Manager,
};
use local_ip_address::local_ip;
use sysinfo::{CpuExt, CpuRefreshKind, DiskExt, RefreshKind, System, SystemExt};

/// Base hardware implementation with common functionality across platforms
pub struct BaseHardware;

impl BaseHardware {
    /// Create a system instance with CPU information
    pub fn get_system() -> System {
        System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()))
    }

    /// Create a battery manager instance
    pub fn get_battery_manager() -> Result<Manager, battery::Error> {
        Manager::new()
    }

    /// Get the CPU model/brand name
    pub fn cpu_model() -> String {
        Self::get_system().cpus()[0].brand().to_string()
    }

    /// Get the number of CPU cores
    pub fn cpu_cores() -> u32 {
        sys_info::cpu_num().unwrap_or(0)
    }

    /// Get system hostname
    pub fn hostname() -> String {
        sys_info::hostname().unwrap_or_else(|_| UNKNOWN_VALUE.to_string())
    }

    /// Get MAC address of the primary network interface
    pub fn mac_address() -> String {
        mac_address::get_mac_address()
            .ok()
            .flatten()
            .map(|addr| addr.to_string())
            .unwrap_or_else(|| UNKNOWN_VALUE.to_string())
    }

    /// Get CPU cache sizes (L1, L2, L3) in bytes
    pub fn cpu_cache() -> (u64, u64, u64) {
        (
            cache_size::l1_cache_size().unwrap_or(0) as u64,
            cache_size::l2_cache_size().unwrap_or(0) as u64,
            cache_size::l3_cache_size().unwrap_or(0) as u64,
        )
    }

    /// Get local IP address
    pub fn local_ip() -> String {
        local_ip()
            .map(|ip| ip.to_string())
            .unwrap_or_else(|_| UNKNOWN_VALUE.to_string())
    }

    /// Check if the system has a battery
    pub fn has_battery() -> bool {
        Self::get_battery_manager()
            .map(|manager| {
                manager
                    .batteries()
                    .map(|mut batteries| batteries.any(|b| b.is_ok()))
                    .unwrap_or(false)
            })
            .unwrap_or(false)
    }

    /// Get memory information (total, free) in bytes
    pub fn memory_info() -> (u64, u64) {
        sys_info::mem_info()
            .map(|mem| (mem.total, mem.free))
            .unwrap_or((0, 0))
    }

    /// Get disk information (total, available) in bytes
    pub fn disk_info() -> (u64, u64) {
        let mut system = System::new_all();
        system.refresh_all();

        system
            .disks()
            .first()
            .map(|disk| (disk.total_space(), disk.available_space()))
            .unwrap_or((0, 0))
    }

    /// Get battery information (health ratio, full energy)
    pub fn battery_info() -> (Ratio, Energy) {
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

    /// Get external IP address
    pub fn external_ip() -> String {
        reqwest::blocking::Client::new()
            .get("https://api.ipify.org")
            .send()
            .and_then(|response| response.text())
            .unwrap_or_else(|_| UNKNOWN_VALUE.to_string())
    }
}

/// Trait for platform-specific hardware operations
pub trait PlatformSpecificHardware {
    /// Get device serial number (platform-specific implementation)
    fn device_serial() -> String;

    /// Get system manufacturing date (platform-specific implementation)
    fn manufactured_date() -> String;

    /// Get system manufacturer name (platform-specific implementation)
    fn system_manufacturer() -> String;

    /// Get system product/model name (platform-specific implementation)
    fn system_product_name() -> String;
}
