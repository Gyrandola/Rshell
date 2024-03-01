use std::process::ExitCode;
use std::string;
use std::io;
use tokenizers::tokenizer::{Result, Tokenizer};

fn main() -> ExitCode{
    // Config


    let encoding = tokenizer.encode("Hey there!", false)?;
    println!("{:?}", encoding.get_tokens());
    // Commands
    rshell_loop();
            //rshell_read_line(); // DEBUG
    // Cleanup

    ExitCode::SUCCESS
}

fn rshell_loop() {
    let mut line: String;
    let args: Vec<String> = vec![String::new(); 126];// char **args
    let mut status: i32 = -1;

    while (status != -1){
        line = rshell_read_line();
        //args = rshell_parse_line(line);

    }
}

// implemented by reading every char
// not elegant but works.
// uses libc crate. possibly unsafe
/*fn rshell_read_line() -> String{
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
}*/

// Implemented by reading the entire line.

fn rshell_read_line() -> String{
    let mut line: String = Default::default();
    let stdin = io::stdin();
    
    // Read line
    stdin.read_line(&mut line).expect("ERROR: rshell_read_line()");

    print!("\nRead line: {}",line); //DEBUG
    return line;
}

// split line and return vector
fn rshell_parse_lines(line: String) -> Vec<String>{

}
