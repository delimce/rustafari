#[path="../../drivers/hardware.rs"]
mod hardware;

#[path="../shared/formats.rs"]
mod formats;

pub fn show_hdw_info() {

    println!("OS Type: {}", hardware::get_os_type());
    println!("OS name: {}", hardware::get_os_name());
    println!("Kernel: {}", hardware::get_os_version());
    println!("CPU Num: {}", hardware::get_cpu_num());
    println!("Total RAM: {}", formats::kb_to_gb( hardware::get_mem_total()));
    println!("Total disc size: {} ",formats::kb_to_gb(hardware::get_disk_size()));
    println!("Hostname: {}", hardware::get_host_name());


}