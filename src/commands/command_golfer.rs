extern crate tokio;

use std::collections::HashMap;

use crate::helpers::sensitive_info_input::sensitive_info_input;
use crate::api::create_golfer_handler::create_golfer_handler;

struct CLICommandGolfer {
    name: String,
    description: String,
    callback: Box<dyn FnMut(&[&str])>
}


fn create_golfer(args: &[&str]) {
    if args.len() < 2 {
        println!("Not enough arguments supplied with the 'golfer create' command.");
        println!("USAGE: golfer create [username] [email address]");
        println!("See API spec for more details: https://samw94.github.io/gogolf-api/");
    } else {
        let username = args[0];
        let email_address = args[1];
        let sensitive_info_name = format!("password for {username}");
        let password = sensitive_info_input(&sensitive_info_name);
        
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
            create_golfer_handler(username, email_address, &password).await.unwrap();
        });
    }
}

fn get_golfer_commands() -> HashMap<&'static str, CLICommandGolfer> {
    return HashMap::from([
        ("create", CLICommandGolfer{
            name: "create".into(),
            description: "Creates a golfer in the gogolf database".into(),
            callback: Box::new(create_golfer),
        })
    ])
}

fn run_golfer_command(cmd: &mut CLICommandGolfer, args: &[&str]) {
    (cmd.callback)(args);
}

pub fn command_golfer(args: &[&str]) {
    let mut golfer_cmds = get_golfer_commands();
    if args.len() == 0 {
        println!("No arguments supplied with the 'golfer' command. The 'gogolf golfer' command takes the following arguments:");
        for (_key, golfer_command) in get_golfer_commands() {
            println!("{command_name} - {command_description}", command_name = golfer_command.name, command_description = golfer_command.description);
        }
    } else {
        let golfer_command_name = args[0];
        if golfer_cmds.contains_key(golfer_command_name) {
            if args.len() > 1 {
                let golfer_cmd = golfer_cmds.get_mut(golfer_command_name).unwrap();
                let golfer_cmd_arguments = &args[1..];
                run_golfer_command(golfer_cmd, golfer_cmd_arguments);
            } else {
                let golfer_cmd = golfer_cmds.get_mut(golfer_command_name).unwrap();
                let golfer_cmd_arguments = &[];
                run_golfer_command(golfer_cmd, golfer_cmd_arguments);
            }
        } else {
            println!("golfer {golfer_command_name} is not a valid command - run 'golfer help' to see a list of valid commands.");
        }
    return;
    }
}