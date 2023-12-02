use super::{draw::{show_welcome, self}, hw::show_hdw_info, hw::show_network_info};

pub fn load() {
    show_welcome();
    show_hdw_info();
    draw::show_separator();
    show_network_info();
}