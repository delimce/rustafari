#[path = "../../drivers/hardware.rs"]
mod hardware;

#[path = "../shared/formats.rs"]
mod formats;

pub fn show_hdw_info() {
    println!("CPU Num: {}", hardware::get_cpu_num());
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
}
