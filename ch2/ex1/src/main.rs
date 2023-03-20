use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // Generate a secret number
    let secret_num = rand::thread_rng().gen_range(1..=100);


    // Give Introduction to user and ask him to input a number
    println!("Guess the number!");
    
    
    // Enter loop until the user has guessed the number
    loop{
        // Ask user to input a number
        println!("Please enter your number:");
        let mut guess = String::new();
        // Read the number
        io::stdin()
           .read_line(&mut guess)
           .expect("Failed to read line");
        // Convert string to int
        let guess: i8 = guess.trim().parse().expect("Please type a number!");
        println!("You have guessed: {guess}");

        // Evaluate the guessed number
        match guess.cmp(&secret_num){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You guessed!");
                break;
            }
        }
    }
}
