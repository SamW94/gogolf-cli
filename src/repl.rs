use std::collections::HashMap;
use rustyline::Editor;
use rustyline::error::ReadlineError;

use crate::commands::command_help::command_help;
use crate::commands::command_golfer::command_golfer;

pub struct CLICommand {
    pub name: String,
    pub description: String,
    pub callback: Box<dyn FnMut(&[&str])>
}

pub fn get_commands() -> HashMap<&'static str, CLICommand> {
    return HashMap::from([
        ("help", CLICommand{
            name: "help".into(),
            description: "Displays this help message".into(),
            callback: Box::new(command_help),
        }),
        ("golfer", CLICommand{
            name: "golfer".into(),
            description: "Commands for creating/finding/updating and deleting golfers".into(),
            callback: Box::new(command_golfer),
        })
    ])
}

fn run_command(cmd: &mut CLICommand, args: &[&str]) {
    (cmd.callback)(args);
}

pub fn clean_input(line: &str) -> Vec<&str> {
    let input_vector: Vec<&str> = line.split(' ').collect();
    return input_vector;
}

pub fn start_repl() {
    let mut rl = Editor::<()>::new();
    let mut cmds = get_commands();
    loop {
        let readline = rl.readline("gogolf> ");
        match readline {
            Ok(line) => {
                let lower_line = line.to_lowercase();
                let trimmed_line = &lower_line.trim();
                let input = clean_input(&trimmed_line); 
                if input[0] == String::from("exit") {
                    println!("Exiting...");
                    break;
                }
                if line.is_empty() {
                    continue;
                }
                let command_name = input[0];
                if cmds.contains_key(command_name) {
                    if input.len() > 1 {
                    let cmd = cmds.get_mut(command_name).unwrap();
                    let command_arguments = &input[1..];
                    run_command(cmd,&command_arguments);
                    } else {
                    let cmd = cmds.get_mut(command_name).unwrap();
                    let command_arguments = &[];
                    run_command(cmd,command_arguments);
                    }
                    
                } else {
                    println!("{command_name} is not a valid command - run 'help' to see a list of gogolf commands.")
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Interrupt received from keyboard, exiting...");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    
    }
}
