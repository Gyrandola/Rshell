use fs::read_dir;
use std::process::{Command, exit};
use std::{env, fs, io};
use std::fs::File;
use std::path::{Path, PathBuf};

// This HAS to be updated if you want to add a built-in.
const BUILTIN_COMMANDS: &[&str] = &["help", "exit", "cd", "mkdir", "deldir", "del", "create", "dir"];


fn main(){
    rshell_loop();
    exit(0);
}

/// The main loop of Rshell.
///
/// This function continuously prompts the user for input, parses commands,
/// executes them, and handles any errors that occur.
///
/// It performs the following steps:
///
/// 1. Prints the current working directory to the console.
/// 2. Reads a line of input from the user.
/// 3. Tokenizes the input into individual command arguments.
/// 4. Attempts to execute the command using the `rshell_execute` function.
/// 5. If the execution fails, prints an error message to the console.
/// 6. Repeats the loop for the next command.
fn rshell_loop() {
    let mut line: String;
    let mut args: Vec<String>;

    loop{
        eprint!("\n{}> ",env::current_dir().unwrap().display());

        line = rshell_read_line();

        args = rshell_tokenize(line);

        if let Err(err) = rshell_execute(args) {
            eprintln!("Error: {}", err);
        }

    }
}

/// Reads a line of input from the user.
///
/// This function prompts the user for input using the standard input stream
/// (`stdin`) and reads the entered line into a `String`. If an error occurs
/// during the reading process, it panics with an error message.
///
/// Returns the read line as a `String`.
fn rshell_read_line() -> String{
    let mut line: String = Default::default();
    let stdin = io::stdin();

    stdin.read_line(&mut line).expect("ERROR: rshell_read_line()");

    return line;
}

/// Splits a line of input into tokens (words).
///
/// This function takes a `String` containing a line of input and splits it into
/// individual words (tokens) based on whitespace characters, including spaces,
/// tabs, carriage returns, newlines, and the BEL character (`\x07`).
///
/// 1. It splits the input string using the specified delimiters (`split`).
/// 2. It converts each split slice into an owned `String` (`map(ToOwned::to_owned)`).
/// 3. It filters out any empty strings (`filter(|token| !token.is_empty())`).
/// 4. It collects the resulting tokens into a `Vec<String>` (`collect()`).
///
/// Returns a vector of tokens, where each token is a `String`.
fn rshell_tokenize(line: String) -> Vec<String> {
    let tokens: Vec<String> = line
        .split(&[' ', '\t', '\r', '\n', '\x07'])
        .map(ToOwned::to_owned)
        .filter(|token| !token.is_empty())
        .collect();

    return tokens;
}

/// Handles built-in commands within the Rshell.
///
/// This function takes a mutable slice of `String` arguments and checks the first
/// element to determine the requested built-in command. It then performs the
/// corresponding action using appropriate methods from the standard library.
///
/// Supported built-in commands are:
///
/// - `help`: Provides help information for Rshell commands.
/// - `exit`: Terminates the shell.
/// - `cd`: Changes or shows the current working directory.
/// - `mkdir`: Creates a new directory or a series of directories.
/// - `deldir`: Deletes a directory and its contents.
/// - `del`: Deletes a file.
/// - `create`: Creates a file.
/// - `dir`: Lists the contents of a directory.
///
/// Returns `Ok(())` on success, or an `Err(io::Error)` if an error occurs.
fn rshell_builtin(args: &mut [String]) -> Result<(), io::Error> {

    match args[0].as_str() {

        "help" =>{
            if !args.get_mut(1).is_some() {
                println!("HELP            provide more information");
                println!("EXIT            exit shell");
                println!("CD              change or show current working directory");
                println!("MKDIR           create new directories");
                println!("DELDIR          delete a directory and all its contents");
                println!("DEL             delete a file");
                println!("CREATE          create a file");
                println!("DIR             visualize directory");
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
                        println!("allows you to change or show the current working directory");
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

                    "dir" => {
                        println!("allows you to visualize a directory");
                        println!("dir [PATH] -> visualize directory given path");
                        println!("dir -> visualize current directory");
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

        "dir" => {
            let dir : PathBuf;

            if !args.get_mut(1).is_some(){
                dir = PathBuf::from(".");
            }else{
                dir = PathBuf::from(&args[1]);
            }

            let entries = read_dir(dir)?;

            println!("TYPE                NAME");
            for entry in entries {
                let entry = entry?;
                let file_type = entry.file_type()?;

                if file_type.is_dir() {
                    println!("directory               {}", entry.path().display());
                } else if file_type.is_file() {
                    println!("file                    {}", entry.path().display());
                }
            }

            Ok(())
        }

        // Ugly
        _ => Err(io::Error::new(io::ErrorKind::NotFound, "command not found")),
    }
}

/// Handles the command as a Built-in or an external one.
///
/// This function takes a vector of `String` arguments representing the command
/// and its arguments. It performs the following steps:
///
/// 1. Checks for empty commands and returns `Ok(())` if no command is provided.
/// 2. Iterates through the `BUILTIN_COMMANDS` list to check if the first argument
///    matches a built-in command.
/// 3. If a match is found, calls `rshell_builtin` to execute the built-in command
///    and returns its result.
/// 4. If the command is not a built-in, calls `rshell_launch` to execute it as
///    an external program and returns its result.
///
/// Returns `Ok(())` on successful execution, or an `Err(io::Error)` if an error
/// occurs during command execution.
fn rshell_execute(mut args : Vec<String>) ->Result<(), io::Error>{

    if !args.get_mut(0).is_some() {
        return Ok(());
    }

    for command in BUILTIN_COMMANDS.iter(){
        if args[0] == *command{
            return rshell_builtin(&mut args[0..]);
        }
    }

    rshell_launch(args)
}

/// Executes a command using the provided arguments and handles potential errors.
///
/// This function takes a vector of strings (`args`) representing the command and
/// its arguments. It then attempts to execute it.
///
/// It performs the following steps:
///
/// 1. Creates a new `Command` instance with the first element of `args` as
///    the program name.
/// 2. Adds the remaining elements of `args` as arguments to the `Command`.
/// 3. Executes the command and captures the output using `command.output()`.
/// 4. If the command succeeds:
///    - Prints the standard output to the console (if any).
/// 5. If the command fails:
///    - Prints the standard error message to the console.
///
/// Returns `Ok(())` if the command executes successfully, or an `Err(io::Error)`
/// containing the error details if an error occurs during execution.
fn rshell_launch(args : Vec<String>) ->Result<(), io::Error>{
    let mut command = Command::new(&args[0]);
    command.args(&args[1..]);

    let output = command.output()?;

    if !output.stdout.is_empty() {
        print!("{}", String::from_utf8_lossy(&output.stdout)); // Use print! for raw output
    }

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        println!("{}", error_message);
    }

    Ok(())
}

