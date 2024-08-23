extern crate mac_address;
extern crate sys_info;

use local_ip_address::local_ip;
use sysinfo::{DiskExt, System, SystemExt};

trait HardwareInfo {
    fn get_cpu_num(&self) -> u32;
    fn get_host_name(&self) -> String;
    fn get_mac_address(&self) -> String;
    fn get_local_ip_address(&self) -> String;
    fn get_external_ip_address(&self) -> String;
}

trait HardwareChange {
    fn get_mem_info(&self) -> (u64, u64);
    fn get_disk_info(&self) -> (u64, u64);
}

pub struct Hardware;

impl HardwareInfo for Hardware {
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

    fn get_local_ip_address(&self) -> String {
        let my_local_ip = local_ip();
        my_local_ip.unwrap().to_string()
    }

    fn get_external_ip_address(&self) -> String {
        "Not implemented".to_string()
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
}

pub fn get_cpu_num() -> u32 {
    Hardware.get_cpu_num()
}

pub fn get_mem_total() -> u64 {
    Hardware.get_mem_info().0
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

// HardwareChange
pub fn get_mem_free() -> u64 {
    Hardware.get_mem_info().1
}

pub fn get_disk_free() -> u64 {
    Hardware.get_disk_info().1
}
