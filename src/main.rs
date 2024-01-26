use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=10);
    
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
    
    println!("The secret number is: {secret_number}");

    match guess.trim().parse::<i32>() {
        Ok(num) => {
            if num == secret_number {
                println!("Which matches!");
            } else {
                println!("Which doesn't match!");
            }
        },
        Err(_) => println!("Please enter a valid number."),
    }
}
