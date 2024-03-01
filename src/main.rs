use std::process::{Command, ExitCode};
use std::{env, io};
use std::path::Path;

const BUILTIN_COMMANDS: &[&str] = &["cd", "exit"];


fn main() -> ExitCode{
    // Config

    // Commands
    rshell_loop();

    // Cleanup

    ExitCode::SUCCESS
}

fn rshell_loop() {
    let mut line: String;
    let mut args: Vec<String>;// char **args

    loop{
        eprint!("{}> ",env::current_dir().unwrap().display());
        line = rshell_read_line();
        args = rshell_tokenize(line);
        if let Err(err) = rshell_execute(args) {
            eprintln!("Error: {}\n", err);
            //break;
        }

    }
}

// Read the entire line from stdin
fn rshell_read_line() -> String{
    let mut line: String = Default::default();
    let stdin = io::stdin();
    
    // Read line
    stdin.read_line(&mut line).expect("ERROR: rshell_read_line()\n");

    //print!("Read line: {}",line); //DEBUG
    return line;
}

// Tokenize line according to preset delimiters
// We don't yet know which token does/mean what
// Return: tokens vector
fn rshell_tokenize(line: String) -> Vec<String> {
    let tokens: Vec<String> = line
        .split(&[' ', '\t', '\r', '\n', '\x07'])
        .map(ToOwned::to_owned)
        .collect();

    //println!("Tokens: {}", tokens.join(", ")); //DEBUG

    return tokens;
}

fn rshell_launch(args : Vec<String>) ->Result<(), std::io::Error>{
    // Add arguments
    let mut command = Command::new(&args[0]);
    command.args(&args[1..]);

    // Execute
    let output = command.output()?;

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        println!("Command failed: {}\n", error_message);
    }

    Ok(())
}

//Handle our shell builtins
fn rshell_builtin(args: &[String]) -> Result<(), std::io::Error> {
    match args[0].as_str() {
        "cd" => {
            if args[1].is_empty() {
                eprintln!("Error: expected argument to cd\n")
            }else{
                let root = Path::new(&args[1]);
                if root.exists(){
                    env::set_current_dir(&root).expect("Failed to change directory\n");
                }else{
                    eprintln!("Error: directory '{}' does not exist\n", root.display());
                }

            }
            Ok(())
        }

        "exit" =>{
            std::process::exit(0);
        }

        //I hate this
        _ => Err(std::io::Error::new(std::io::ErrorKind::NotFound, "builtin not found\n")),
    }
}

fn rshell_execute(args : Vec<String>) ->Result<(), std::io::Error>{

    // Empty command
    if args[0].is_empty(){
        return Ok(());
    }

    // Look for builtins and execute them
    for command in BUILTIN_COMMANDS.iter(){
        if args[0] == *command{
            return rshell_builtin(&args[0..]);
        }
    }

    rshell_launch(args)
}

