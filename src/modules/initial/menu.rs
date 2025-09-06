use colored::*;

pub fn show_menu() {
    println!("{}", "1. Refresh".blue().bold());
    println!("{}", "2. Scan".green().bold());
    println!("{}", "3. Exit".red().bold());
}

pub fn read_menu_option() -> String {
    let mut option = String::new();
    std::io::stdin().read_line(&mut option).unwrap();
    option.trim().to_string()
}
