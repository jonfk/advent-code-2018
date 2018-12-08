extern crate common;

use std::fmt;
use std::fs;

fn main() {
    println!("Hello, day 5!");


    //let input = common::open_input("day5/input.txt").expect("failed to get input");

    let input = fs::read_to_string("day5/input.txt").expect("failed to get input");

    let mut particles = Particles::new(&input);
    particles.react_completely();

    //println!("particles: {}", particles);
    println!("len particles: {}", particles.particles.len());

    let most_reactive_polymer = find_most_reactive_polymer(&input);
    println!("len most reactive polymer: {}", most_reactive_polymer.len());
}

pub fn find_most_reactive_polymer(input: &str) -> Particles {
    let mut most_reactive = Particles::new(&input);

    for ascii_code in 65 ..= 90 {
        let ch = char::from(ascii_code);
        println!("trying {}", ch);
        let mut new_particles = Particles::new(&input);
        new_particles.extract(ch);
        new_particles.react_completely();
        if new_particles.len() < most_reactive.len() {
            most_reactive = new_particles;
        }
    }

    most_reactive
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
            particles: input.chars().filter(|c| !c.is_whitespace()).map(|c| Particle::UnReacted(c)).collect(),
        }
    }

    pub fn len(&self) -> usize {
        self.particles.len()
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

    pub fn extract(&mut self, ch: char) {
        self.particles = self.particles.clone().into_iter().filter(|x| {
            if let Particle::UnReacted(unreacted_ch) = x {
                !unreacted_ch.eq_ignore_ascii_case(&ch)
            } else {
                true
            }
        }).collect()
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
