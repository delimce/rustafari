use super::{draw::show_welcome, hw::show_hdw_info};

pub fn load() {
    show_welcome();
    show_hdw_info()
}