extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::{thread_rng, Rng};

fn main() {
    println!("Guess the number");
    println!("Please input you guess");
    let rnd = thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", rnd);
    
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(result)  => result,
            Err(_)      => continue
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&rnd) {
            Ordering::Less      => println!("Too small!"),
            Ordering::Equal     => {
                println!("You win!");
                break;
            },
            Ordering::Greater   => println!("Too big!"),
        }
    }
}
