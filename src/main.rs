mod repl;
mod commands {
    pub mod command_golfer;
    pub mod command_help;
    pub mod types;
}
mod helpers {
    pub mod sensitive_info_input;
}
mod api {
    pub mod create_golfer_handler;
}

use crate::repl::start_repl;

fn main() {
    println!("Starting gogolf-cli...");
    start_repl();
}
