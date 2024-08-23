use super::{
    draw::{self, show_welcome},
    hw::show_hdw_info,
    hw::show_network_info,
    sw::show_sw_info,
};

pub fn load() {
    show_welcome();
    show_hdw_info();
    draw::show_separator();
    show_sw_info();
    draw::show_separator();
    show_network_info();
}
