extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Would you like to play a game?");

    let secret_number = rand::thread_rng()
        .gen_range(1, 101);             // range from 1 -> 100 (inclusive, exclusive)

    loop {
        println!("Guess the number.");
        
        let mut guess = String::new();      // mut specifies "mutable", refs immutable by default

        // Could also be: std::io::stdin().read_line(&mut guess), but that's what "use" is for.
        io::stdin().read_line(&mut guess)   // Returns io::Result
            .ok()                           // io::Result.ok() => "OK" or Err
            .expect("Failed to read line"); // if Err, print <msg> (Error handling in Rust)

        // Type conversion == "Type shadowing" in Rust
        // Re-type "guess" from String to uint32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // "_" means we don't care about the error.
        };

        // Smart placement, don't have to specify {} index for multi-ref replacement.
        println!("You guessed: {}", guess);

        // "match" is like "switch -> case", Ordering replaces "if..elif..else"
        // Use "Ordering" for multiple conditions where it makes sense. Ordering is enum.
        match guess.cmp(&secret_number) {
            Ordering::Less      => println!("Too small!"),
            Ordering::Greater   => println!("Too big!"),
            Ordering::Equal     => {
                println!("You win!");
                break; // Break out of the loop if we win.
            }
        }
    }
}
