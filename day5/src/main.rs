extern crate common;

fn main() {
    println!("Hello, day 5!");

    let input = common::open_input();
}

fn run_reaction_cycle(particles: &str) -> String {
    let mut remove_indexes = Vec::new();
    particles.chars().peekable().enumerate().
}

// We can assume the inputs are ascii
fn is_opposite(a: char, b: char) -> bool {
    if a.is_lowercase() {
        a.to_uppercase().nth(0).unwrap() == b
    } else {
        a.to_lowercase().nth(0).unwrap() == b
    }
}
