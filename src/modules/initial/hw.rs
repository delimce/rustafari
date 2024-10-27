#[path = "../../drivers/hardware.rs"]
mod hardware;

#[path = "../shared/formats.rs"]
mod formats;

pub fn show_hdw_info() {
    println!("CPU Model: {}", hardware::get_cpu_model());
    println!("CPU Cores: {}", hardware::get_cpu_num());

    let (l1, l2, l3) = hardware::get_cpu_cache();
    println!(
        "CPU Cache: L1:{}, L2:{}, L3:{}",
        formats::bytes_to_mb(l1),
        formats::bytes_to_mb(l2),
        formats::bytes_to_mb(l3)
    );
    println!(
        "Total RAM: {}",
        formats::kb_to_gb(hardware::get_mem_total())
    );
    println!(
        "Total disc size: {} ",
        formats::bytes_to_gb(hardware::get_disk_size())
    );

    println!(
        "Total disc available: {}",
        formats::bytes_to_gb(hardware::get_disk_free())
    );

    println!("Hostname: {}", hardware::get_host_name());
}

pub fn show_network_info() {
    println!("MAC address: {}", hardware::get_mac_address());
    println!("Local IP address: {}", hardware::get_local_ip_address());
    println!(
        "External IP address: {}",
        hardware::get_external_ip_address()
    );
    println!("Device serial number: {}", hardware::get_device_serial());
}
