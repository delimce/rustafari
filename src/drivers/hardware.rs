extern crate battery;
extern crate cache_size;
extern crate mac_address;
extern crate sys_info;

use battery::units::{energy::watt_hour, ratio::percent, Energy, Ratio};
use local_ip_address::local_ip;
use sysinfo::{CpuExt, CpuRefreshKind, DiskExt, RefreshKind, System, SystemExt};

const UNKNOWN_VALUE: &str = "Unknown";

trait HardwareInfo {
    fn get_cpu_name(&self) -> String;
    fn get_cpu_num(&self) -> u32;
    fn get_host_name(&self) -> String;
    fn get_mac_address(&self) -> String;
    fn get_cpu_cache_total(&self) -> (u64, u64, u64);
    fn get_local_ip_address(&self) -> String;
    fn get_device_serial(&self) -> String;
}

trait HardwareChange {
    fn get_mem_info(&self) -> (u64, u64);
    fn get_disk_info(&self) -> (u64, u64);
    fn get_battery_info(&self) -> (Ratio, Energy);
}

pub struct Hardware;

impl HardwareInfo for Hardware {
    fn get_cpu_name(&self) -> String {
        let s =
            System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()));
        let cpu_name = s.cpus()[0].brand();
        cpu_name.to_string()
    }

    fn get_cpu_num(&self) -> u32 {
        sys_info::cpu_num().unwrap()
    }

    fn get_host_name(&self) -> String {
        sys_info::hostname().unwrap()
    }

    fn get_mac_address(&self) -> String {
        let interfaces = mac_address::get_mac_address().unwrap();
        let mut mac_address = String::new();
        for interface in interfaces.iter() {
            mac_address.push_str(&interface.to_string());
        }
        mac_address
    }

    fn get_cpu_cache_total(&self) -> (u64, u64, u64) {
        let cache1 = cache_size::l1_cache_size();
        let cache2 = cache_size::l2_cache_size();
        let cache3 = cache_size::l3_cache_size();
        //return only if this values are numbers
        (
            cache1.unwrap_or(0) as u64,
            cache2.unwrap_or(0) as u64,
            cache3.unwrap_or(0) as u64,
        )
    }

    fn get_local_ip_address(&self) -> String {
        let my_local_ip = local_ip();
        my_local_ip.unwrap().to_string()
    }

    fn get_device_serial(&self) -> String {
        let serials = mid::data("mySecretKey").unwrap();
        let binding: String = UNKNOWN_VALUE.to_string();
        let serial_data = serials.result.get(1).unwrap_or(&binding);
        serial_data.to_string()
    }
}

impl HardwareChange for Hardware {
    fn get_mem_info(&self) -> (u64, u64) {
        let mem = sys_info::mem_info().unwrap();
        (mem.total, mem.free)
    }

    fn get_disk_info(&self) -> (u64, u64) {
        let mut system = System::new_all();
        system.refresh_all();
        let disk = system.disks();
        (disk[0].total_space(), disk[0].available_space())
    }

    fn get_battery_info(&self) -> (Ratio, Energy) {
        let manager = battery::Manager::new().unwrap();
        let batteries = manager.batteries().unwrap();
        let mut battery_info = (Ratio::new::<percent>(0.0), Energy::new::<watt_hour>(0.0));
        for battery in batteries {
            if let Ok(battery) = battery {
                battery_info = (battery.state_of_health(), battery.energy_full());
            }
        }
        battery_info
    }
}

pub fn get_cpu_model() -> String {
    Hardware.get_cpu_name()
}

pub fn get_cpu_num() -> u32 {
    Hardware.get_cpu_num()
}

pub fn get_mem_total() -> u64 {
    Hardware.get_mem_info().0
}

pub fn get_cpu_cache() -> (u64, u64, u64) {
    Hardware.get_cpu_cache_total()
}

pub fn get_disk_size() -> u64 {
    Hardware.get_disk_info().0
}

pub fn get_host_name() -> String {
    Hardware.get_host_name()
}

pub fn get_mac_address() -> String {
    Hardware.get_mac_address()
}

pub fn get_local_ip_address() -> String {
    Hardware.get_local_ip_address()
}

pub fn get_external_ip_address() -> String {
    get_external_ip_address_async()
}

fn get_external_ip_address_async() -> String {
    let http_client = reqwest::blocking::Client::new();
    let url = "https://api.ipify.org";
    let text = http_client.get(url).send().unwrap().text();
    text.unwrap()
}

pub fn _get_mem_free() -> u64 {
    Hardware.get_mem_info().1
}

pub fn get_disk_free() -> u64 {
    Hardware.get_disk_info().1
}

pub fn get_device_serial() -> String {
    Hardware.get_device_serial()
}

pub fn get_battery_info() -> (Ratio, Energy) {
    Hardware.get_battery_info()
}
