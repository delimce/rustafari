pub fn show_welcome() {
    show_blank_line();
    show_banner();
    show_blank_line();
    show_version();
    show_separator();
}

pub fn show_banner() {
    println!(
        r#"    _ __ _   _ ___| |_ __ _ / _| __ _ _ __(_)   ___| (_) ___ _ __ | |_ 
   | '__| | | / __| __/ _` | |_ / _` | '__| |  / __| | |/ _ \ '_ \| __|
   | |  | |_| \__ \ || (_| |  _| (_| | |  | | | (__| | |  __/ | | | |_ 
   |_|   \__,_|___/\__\__,_|_|  \__,_|_|  |_|  \___|_|_|\___|_| |_|\__|"#
    );
}

pub fn show_blank_line() {
    println!();
}

pub fn show_separator() {
    println!("-------------------------------------------------------------------------");
}

pub fn show_version() {
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
}
