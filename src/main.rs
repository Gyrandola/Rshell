use std::process::{Command, exit};
use std::{env, fs, io};
use std::fs::File;
use std::path::Path;

const BUILTIN_COMMANDS: &[&str] = &["help", "exit", "cd", "mkdir", "deldir", "del", "create"];


fn main(){
    rshell_loop();


    exit(0);
}

fn rshell_loop() {
    let mut line: String;
    let mut args: Vec<String>;

    loop{
        // Print current dir every line
        eprint!("\n{}> ",env::current_dir().unwrap().display());

        // Read raw line from input
        line = rshell_read_line();

        // Tokenize it
        args = rshell_tokenize(line);

        // Execute it and handle errors
        if let Err(err) = rshell_execute(args) {
            eprintln!("Error: {}", err);
        }

    }
}

// Read the entire line from stdin
fn rshell_read_line() -> String{
    let mut line: String = Default::default();
    let stdin = io::stdin();
    
    // Read line
    stdin.read_line(&mut line).expect("ERROR: rshell_read_line()");

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
        .filter(|token| !token.is_empty()) // Filter after mapping
        .collect();

    //println!("Debug: Tokenized line: {:?}", tokens); // FOR DEBUGGING

    return tokens;
}

fn rshell_launch(args : Vec<String>) ->Result<(), io::Error>{
    // Add arguments
    let mut command = Command::new(&args[0]);
    command.args(&args[1..]);

    //println!("Command: {:?}",command); // FOR DEBUGGING

    // Execute
    let output = command.output()?;

    // Print output
    if !output.stdout.is_empty() {
        print!("{}", String::from_utf8_lossy(&output.stdout)); // Use print! for raw output
    }

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        println!("{}", error_message);
    }

    Ok(())
}

//Handle our shell builtins
fn rshell_builtin(args: &mut [String]) -> Result<(), io::Error> {

    match args[0].as_str() {

        "help" =>{
            // If argument doesn't exit
            if !args.get_mut(1).is_some() {
                println!("HELP:         provide more information");
                println!("EXIT:         exit shell");
                println!("CD:           change  or visualize current working directory");
                println!("MKDIR:        create new directories");
                println!("DELDIR:       delete a directory and all its contents");
                println!("DEL:          delete a file");
                println!("CREATE:          create a file");
                return Ok(());
            }else{
                match args[1].as_str(){

                    "help" => {
                        println!("provides help information for Rshell commands");
                        println!("help [COMMAND] -> specific command information");
                        return Ok(());
                    }

                    "exit" => {
                        println!("terminates rshell process with successful exit code");
                        return Ok(());
                    }

                    "cd" => {
                        println!("allows you to change or visualize the current working directory");
                        println!("cd [PATH] -> move to specified directory path");
                        println!("cd .. -> move to parent directory");
                        return Ok(());
                    }

                    "mkdir" => {
                        println!("creates a new directory or a series of directories given a path");
                        println!("mkdir [PATH]");
                        return Ok(());
                    }

                    "del" => {
                        println!("delete a file given a path");
                        println!("del [PATH]");
                        return Ok(());
                    }

                    "create" => {
                        println!("create a file given a path");
                        println!("create [PATH]");
                        return Ok(());
                    }
                    _ => Err(io::Error::new(io::ErrorKind::NotFound, "unexpected argument for help")),
                }?;
            }
            Ok(())
        }

        "exit" =>{
            exit(0);
        }

        "cd" => {
            if !args.get_mut(1).is_some(){
                eprint!("{}\n",env::current_dir().unwrap().display());
            }else{
                let root = Path::new(&args[1]);

                if root.exists(){
                    let path = args[1..].join(" ");
                    env::set_current_dir(Path::new(&path)).expect("Failed to change directory");
                }else{
                    eprintln!("Error: cannot find specified path");
                }
            }
            Ok(())
        }

        "mkdir" => {
            if !args.get_mut(1).is_some() {
                eprintln!("Error: expected argument to mkdir");
            }else{
                let path = args[1..].join(" ");
                fs::create_dir_all(path).expect("Failed to create directories");
            }

            Ok(())
        }


        "deldir" => {
            if !args.get_mut(1).is_some() {
                eprintln!("Error: expected argument to deldir");
            }else{
                let path = args[1..].join(" ");
                fs::remove_dir_all(path).expect("Failed to delete directory");
            }

            Ok(())
        }

        "del" => {
            if !args.get_mut(1).is_some() {
                eprintln!("Error: expected argument to del");
            }else{
                let path = args[1..].join(" ");
                fs::remove_file(path).expect("Failed to delete file");
            }

            Ok(())
        }

        "create" => {
            if !args.get_mut(1).is_some() {
                eprintln!("Error: expected argument to create");
            }else{
                let path = args[1..].join(" ");
                File::create(path).expect("Failed to create file");
            }

            Ok(())
        }

        // Ugly
        _ => Err(io::Error::new(io::ErrorKind::NotFound, "command not found")),
    }
}

fn rshell_execute(mut args : Vec<String>) ->Result<(), io::Error>{

    // Empty command
    if args[0].is_empty(){
        return Ok(());
    }

    // Look for builtins and execute them
    for command in BUILTIN_COMMANDS.iter(){
        if args[0] == *command{
            return rshell_builtin(&mut args[0..]);
        }
    }

    rshell_launch(args)
}

