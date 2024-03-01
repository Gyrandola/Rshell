use std::process::ExitCode;
use libc::{c_int, EOF, getchar};
use std::string;

fn main() -> ExitCode{
    // Config

    // Commands
    shell_loop();
    shell_read_line(); // DEBUG
    // Cleanup

    ExitCode::SUCCESS
}

fn shell_loop() {
    let mut line: &str;
    let args: Vec<String> = vec![String::new(); 126];// char **args
    let mut status: i32;

}

// implemented by reading every char
// not elegant but works.
fn shell_read_line() -> String{
    let mut line: String = Default::default();
    let mut character: char;

    loop {
        // Read a character from input
        unsafe {
            // getchar() returns a c_int which we cast to a 8-bit integer (1 byte).
            // We then cast it as an actual character.
            character = char::from(getchar() as u8);
        }

        // Append characters to line
        if(character as i32 == EOF || character == '\n'){
            line.push('\0');
            print!("\nLine read: {}", line); // DEBUG
            return line;
        }else{
            line.push(character);
        }
    }
}

/*// Implemented by reading the entire line. TODO.
fn shell_read_line() -> String{
    let mut line: String = Default::default();



    return line;
}*/
