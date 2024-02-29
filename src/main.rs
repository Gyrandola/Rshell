use std::process::ExitCode;
use libc::{c_int, EOF, getchar};
use std::string;

fn main() -> ExitCode{
    // Config

    // Commands
    shell_loop();
    // Cleanup

    ExitCode::SUCCESS
}

fn shell_loop() {
    let mut line: &str;
    let args: Vec<String> = vec![String::new(); 126];// char **args
    let mut status: i32;

}


/*fn shell_read_line() -> String{
    let mut line: String = Default::default();
    let mut character: char;

    loop {
        // Read a character from input
        unsafe {
            character = char::from(getchar() as u8);
        }

        // Append characters to line
        if(character as i32 == EOF || character == '\n'){
            line.push('\0');
            return line;
        }else{
            line.push(character);
        }

    }
}*/

fn shell_read_line() -> String{
    let mut line: String = Default::default();



    return line;
}
