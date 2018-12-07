
use std::env;
use std::fs;
use std::io::Error;

pub fn open_input(default_file: &str) -> Result<String, Error> {
    let input_file = env::args().nth(1).unwrap_or(default_file.to_owned());
    fs::read_to_string(input_file)
}
