//! Hardware information module with platform-specific implementations
//!
//! This module provides a unified interface for gathering hardware information
//! across different operating systems (macOS and Linux) while keeping the
//! platform-specific code separated for better maintainability.

#![allow(dead_code)]

pub mod base;
pub mod traits;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "linux")]
pub mod linux;

use battery::units::{Energy, Ratio};
use traits::HardwareProvider;

// Re-export the appropriate implementation based on target OS
#[cfg(target_os = "macos")]
use macos::MacOSHardware as PlatformHardware;

#[cfg(target_os = "linux")]
use linux::LinuxHardware as PlatformHardware;

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
compile_error!("Unsupported platform. Only macOS and Linux are supported.");

/// Main Hardware struct that delegates to platform-specific implementations
pub struct Hardware;

impl Hardware {
    /// Get the CPU model/brand name
    pub fn cpu_model() -> String {
        PlatformHardware::cpu_model()
    }

    /// Get the number of CPU cores
    pub fn cpu_cores() -> u32 {
        PlatformHardware::cpu_cores()
    }

    /// Get system hostname
    pub fn hostname() -> String {
        PlatformHardware::hostname()
    }

    /// Get MAC address of the primary network interface
    pub fn mac_address() -> String {
        PlatformHardware::mac_address()
    }

    /// Get CPU cache sizes (L1, L2, L3) in bytes
    pub fn cpu_cache() -> (u64, u64, u64) {
        PlatformHardware::cpu_cache()
    }

    /// Get local IP address
    pub fn local_ip() -> String {
        PlatformHardware::local_ip()
    }

    /// Get device serial number
    pub fn device_serial() -> String {
        PlatformHardware::device_serial()
    }

    /// Check if the system has a battery
    pub fn has_battery() -> bool {
        PlatformHardware::has_battery()
    }

    /// Get memory information (total, free) in bytes
    pub fn memory_info() -> (u64, u64) {
        PlatformHardware::memory_info()
    }

    /// Get disk information (total, available) in bytes
    pub fn disk_info() -> (u64, u64) {
        PlatformHardware::disk_info()
    }

    /// Get battery information (health ratio, full energy)
    pub fn battery_info() -> (Ratio, Energy) {
        PlatformHardware::battery_info()
    }

    /// Get external IP address
    pub fn external_ip() -> String {
        PlatformHardware::external_ip()
    }

    /// Get system manufacturing date
    pub fn manufactured_date() -> String {
        PlatformHardware::manufactured_date()
    }

    /// Get system manufacturer name
    pub fn system_manufacturer() -> String {
        PlatformHardware::system_manufacturer()
    }

    /// Get system product/model name
    pub fn system_product_name() -> String {
        PlatformHardware::system_product_name()
    }
}

// Public API functions - maintaining backward compatibility
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
