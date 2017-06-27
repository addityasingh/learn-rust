extern crate communicator;
use communicator::client::{connect};

fn main() {
    let strg: i32 = connect();
    println!("bfdjvk {:?}", strg);
}