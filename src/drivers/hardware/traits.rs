use battery::units::{Energy, Ratio};

/// Common interface for hardware information gathering
pub trait HardwareProvider {
    /// Get the CPU model/brand name
    fn cpu_model() -> String;

    /// Get the number of CPU cores
    fn cpu_cores() -> u32;

    /// Get system hostname
    fn hostname() -> String;

    /// Get MAC address of the primary network interface
    fn mac_address() -> String;

    /// Get CPU cache sizes (L1, L2, L3) in bytes
    fn cpu_cache() -> (u64, u64, u64);

    /// Get local IP address
    fn local_ip() -> String;

    /// Get device serial number
    fn device_serial() -> String;

    /// Check if the system has a battery
    fn has_battery() -> bool;

    /// Get memory information (total, free) in bytes
    fn memory_info() -> (u64, u64);

    /// Get disk information (total, available) in bytes
    fn disk_info() -> (u64, u64);

    /// Get battery information (health ratio, full energy)
    fn battery_info() -> (Ratio, Energy);

    /// Get external IP address
    fn external_ip() -> String;

    /// Get system manufacturing date
    fn manufactured_date() -> String;

    /// Get system manufacturer name
    fn system_manufacturer() -> String;

    /// Get system product/model name
    fn system_product_name() -> String;
}

/// Common constants used across implementations
pub mod constants {
    pub const UNKNOWN_VALUE: &str = "Unknown";
}
