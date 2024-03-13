// Importing the rand crate/library
use rand::Rng;

// Importing the io module from the standard library
use std::io;

fn main() {
    println!("Hello, world!");

    // Generating a random number between 1 and 10
    let secret_number = rand::thread_rng().gen_range(1..11);

    // Printing the secret number
    let mut attempts = 0;
}
