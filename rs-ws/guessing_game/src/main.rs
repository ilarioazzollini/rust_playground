use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    // Initialization
    // range expression (start..=end) is inclusive on the lower and upper bounds
    let secret_number: u32 = rand::thread_rng().gen_range(0..=100);
    let max_attempts: i32 = 10;
    let mut count: i32 = 0;

    // Intro
    println!("\n-----------------------------");
    println!("Welcome to the Guessing Game!\n");
    println!("Rules:");
    println!("- I have generated a random integer number within the range of 0 to 100 inclusive");
    println!("- You have {max_attempts} attempts to guess the number");
    println!(
        "- For each attempt, I'll tell you whether your guess is lower, higher, or the exact number"
    );
    println!("-----------------------------");

    loop {
        count += 1;

        if count == (max_attempts + 1) {
            println!("\nYOU LOST! (Max number of attempts reached)");
            break;
        }

        println!("\nAttempt #{count}. Please input your guess and press <Enter>.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Shadowing previous variable guess with the new variable guess (different types)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input not valid, you should write a positive integer number.");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("LOWER!"),
            Ordering::Greater => println!("HIGHER!"),
            Ordering::Equal => {
                println!("YOU WON!");
                break;
            }
        }
    }

    // Outro
    println!("\n-------------------");
    println!("End of the program.");
    println!("-------------------");
}
