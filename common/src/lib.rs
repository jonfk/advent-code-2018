
use std::env;
use std::fs;
use std::io::Error;

pub fn open_input() -> Result<String, Error> {
    let input_file = env::args().nth(0).unwrap_or("input.txt".to_owned());
    fs::read_to_string(input_file)
}
