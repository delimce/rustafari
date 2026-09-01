#[path = "../../drivers/software.rs"]
mod software;
use colored::*;

pub fn show_sw_info() {
    println!("OS Type: {}", software::get_os_type());
    println!("OS name: {}", software::get_os_name().yellow());
    println!("OS Kernel: {}", software::get_os_version());
    println!("OS Arquitecture: {}", software::get_so_architecture());
}
