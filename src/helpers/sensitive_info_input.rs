extern crate rpassword;

use rpassword::read_password;
use std::io::Write;

pub fn sensitive_info_input(sensitive_info_name: &str) -> String {
    print!("Type a {sensitive_info_name}: ", sensitive_info_name=sensitive_info_name);
    std::io::stdout().flush().unwrap();
    let sensitive_info = read_password().unwrap();
    return sensitive_info;
}