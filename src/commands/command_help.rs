use crate::repl::get_commands;

pub fn command_help(args: &[&str]) {
    if args.len() > 0 {
        println!("The 'help' command takes no arguments.");
    }
    for (_key, command) in get_commands() {
        println!("{command_name} - {command_description}", command_name = command.name, command_description = command.description);
    }
}
