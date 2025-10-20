use rustyline::Editor;
use rustyline::error::ReadlineError;

pub fn repl() {
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline("gogolf> ");
        match readline {
            Ok(line) => {
                if line == String::from("exit") {
                    println!("Exiting...");
                    break;
                }
                if line.is_empty() {
                    continue;
                }
                eval(&line);
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

fn eval(input: &String) {
    let input_vector: Vec<&str> = input.split(' ').collect();
    let command_name: &str = input_vector[0];
    let mut command_arguments: Vec<&str> = vec![];
    if input_vector.len() > 1 {
        for arg in &input_vector[1..] {
            command_arguments.push(arg);
        }
    }

    println!("You entered the command '{}'", command_name);
    for arg in command_arguments {
        println!("You entered the command argument '{}'", arg);
    }
}