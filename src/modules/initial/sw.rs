#[path = "../../drivers/software.rs"]
mod software;

pub fn show_sw_info() {
    println!("OS Type: {}", software::get_os_type());
    println!("OS name: {}", software::get_os_name());
    println!("Kernel: {}", software::get_os_version());
}
