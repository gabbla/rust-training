extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("##############################");
    println!("Guess the number :)");
    println!("##############################");
    println!("Input your number: ");
    
    let rnd = rand::thread_rng().gen_range(1, 101);

    loop {

        // By default the variables in Rust are unmutable
        // new is an associated function of String type (aka static function)
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match  guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&rnd) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
