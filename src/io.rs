// input and output module
// will eventually use web base gui
// for now just use terminal

use std::io;

// function to read a line from the terminal
pub fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}

// function to print a string to the terminal
pub fn print_line(string: String) {
    println!("{}", string);
}

