#[path = "../../drivers/hardware.rs"]
mod hardware;

#[path = "../shared/formats.rs"]
mod formats;

pub fn show_hdw_info() {
    let (l1, l2, l3) = hardware::get_cpu_cache();
    let memory_gb = formats::kb_to_gb(hardware::get_mem_total());
    let disk_total_gb = formats::bytes_to_gb(hardware::get_disk_size());
    let disk_free_gb = formats::bytes_to_gb(hardware::get_disk_free());

    println!("CPU Model: {}", hardware::get_cpu_model());
    println!("CPU Cores: {}", hardware::get_cpu_num());
    println!(
        "CPU Cache: L1:{}, L2:{}, L3:{}",
        formats::bytes_to_mb(l1),
        formats::bytes_to_mb(l2),
        formats::bytes_to_mb(l3)
    );
    println!("Total RAM: {memory_gb}");
    println!("Total disc size: {disk_total_gb}");
    println!("Total disc available: {disk_free_gb}");
    println!("Hostname: {}", hardware::get_host_name());
    println!("Manufacturer: {}", hardware::get_system_manufacturer());
    println!("Product Name: {}", hardware::get_system_product_name());
    println!("Manufactured Date: {}", hardware::get_manufactured_date());

    print_battery_info();
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

fn print_battery_info() {
    if hardware::has_battery() {
        let battery_info = hardware::get_battery_info();
        let battery_percentage = formats::ratio_to_float(battery_info.0);
        println!(
            "Battery life: {}",
            formats::float_to_percent(battery_percentage)
        );
    } else {
        println!("Battery life: N/A");
    }
}
