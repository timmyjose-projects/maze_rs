//! This module provides IO functionality for the whole project.

use std::io::{self, Write};
use std::str::FromStr;

use crate::error::MazeError;
use crate::graphics::renderer;

/// read in the command line arguments, skipping the
/// first one (the program name)
pub fn get_args() -> Vec<String> {
    ::std::env::args().skip(1).collect::<Vec<String>>()
}

/// Read in a non-negative integer from the
/// console
pub fn get_number() -> Option<usize> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    match usize::from_str(input.trim()) {
        Ok(val) => Some(val),
        Err(__) => None,
    }
}

/// print a message on standard output,
/// with no newline
pub fn print_message(message: &str) {
    write!(io::stdout(), "{}", message).unwrap();
    flush();
}

/// print a message on standard output, with a
/// newline
pub fn println_message(message: &str) {
    writeln!(io::stdout(), "{}", message).unwrap();
}

/// print a message on standard output, and quit
/// immediately
pub fn print_message_and_quit(message: &str) {
    writeln!(io::stdout(), "{}", message).unwrap();
    ::std::process::exit(1);
}

pub fn print_error_and_quit(err: Box<MazeError>) {
    writeln!(io::stdout(), "{}", err).unwrap();
    ::std::process::exit(1);
}

/// reset the menu back to the original
/// location so that the page does not
/// scroll down
pub fn adjust_menu_location_on_screen() {
    renderer::delete_current_line();
}

pub fn flush() {
    io::stdout().flush().unwrap();
}
