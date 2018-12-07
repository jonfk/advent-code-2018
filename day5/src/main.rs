extern crate common;

use std::fmt;
use std::fs;

fn main() {
    println!("Hello, day 5!");


    //let input = common::open_input("day5/input.txt").expect("failed to get input");

    let input = fs::read_to_string("day5/input.txt").expect("failed to get input");

    let mut particles = Particles::new(&input);
    particles.react_completely();

    println!("particles: {}", particles);
    println!("len particles: {}", particles.particles.len());
}

pub struct Particles {
    pub particles: Vec<Particle>,
}

#[derive(Copy,Clone, Eq, PartialEq)]
pub enum Particle {
    Reacted,
    UnReacted(char),
}

impl fmt::Display for Particle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Particle::Reacted => {
                write!(f, "*")
            }
            Particle::UnReacted(ch) => {
                write!(f, "{}", ch)
            }
        }
    }
}

impl Particles {
    pub fn new(input: &str) -> Particles {
        Particles{
            particles: input.chars().map(|c| Particle::UnReacted(c)).collect(),
        }
    }

    pub fn react_once(&mut self) {
        {
        let mut iter = self.particles.iter_mut().peekable();
        let mut remove_cur = false;
        while let Some(current) = iter.next() {
            if remove_cur {
                *current = Particle::Reacted;
                remove_cur = false;
            } else {
                if let Some(next) = iter.peek() {
                    if let Particle::UnReacted(cur_char) = current.clone() {
                        if let Particle::UnReacted(next_char) = next {
                            if is_opposite(&cur_char, next_char) {
                                *current = Particle::Reacted;
                                remove_cur = true;
                            }
                        }
                    }
                }
            }
        }
        }

        self.particles = self.particles.clone().into_iter().filter(|x| *x != Particle::Reacted).collect();
    }

    pub fn react_completely(&mut self) {
        let mut old = self.particles.clone();
        self.react_once();
        while old.len() != self.particles.len() {
            old = self.particles.clone();
            self.react_once();
        }
        self.react_once();
    }
}

impl fmt::Display for Particles {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.particles.iter().map(|x| x.fmt(f)).collect()
    }
}

// We can assume the inputs are ascii
fn is_opposite(a: &char, b: &char) -> bool {
    if a.is_lowercase() {
        a.to_uppercase().nth(0).unwrap() == *b
    } else {
        a.to_lowercase().nth(0).unwrap() == *b
    }
}
