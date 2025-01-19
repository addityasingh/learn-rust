extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::{thread_rng, Rng};

pub struct Guess {
    value: u32
}

impl Guess {
    pub fn new (value: u32) -> Guess {
        if !(1..=100).contains(&value) {
            panic!("Expected value between 1 & 100. Got {}", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

fn main() {
    println!("Guess the number");
    println!("Please input you guess");
    let rnd = thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", rnd);
    
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        let guess: Guess = match guess.trim().parse() {
            Ok(result)  => Guess::new(result),
            Err(_)      => continue
        };

        println!("You guessed: {:?}", guess.value());

        match guess.value().cmp(&rnd) {
            Ordering::Less      => println!("Too small!"),
            Ordering::Equal     => {
                println!("You win!");
                break;
            },
            Ordering::Greater   => println!("Too big!"),
        }
    }
}
