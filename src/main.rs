
use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

/// Crate Guessing Game.
/// What is the secret number?

fn main() {

    //! # Main function
    //! Reads the user input and checks if it matches secret number
    //! 
    //! 

    println!("Welcome to guess a number!");

    let secret_num = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_num);

    loop{
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_num){
            Ordering::Less => println!("{}", "Too small".red()),
            Ordering::Greater => println!("{}", "Too big".red()),
            Ordering::Equal => {
                println!("{}", "You got it right!".green());
                break;
            }
        }
    }
}
