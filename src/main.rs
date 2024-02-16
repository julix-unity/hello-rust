use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn guessing_game() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a number, try again!");
                continue;
            },
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}


// Assume we have a struct that represents user data
#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

// Imagine this function is part of a library that parses JSON strings into User structs
// It takes ownership of the input string because parsing can be a consuming operation
fn parse_json_to_user(data: String) -> User {
    // Placeholder for actual JSON parsing logic
    // For demonstration, we'll just construct a User directly
    User {
        name: String::from("Jane Doe"),
        age: 30,
    }
}

fn main() {
    // guessing_game();

    let json_data = String::from(r#"{"name": "Jane Doe", "age": 30}"#);
    let user = parse_json_to_user(json_data); // json_data is moved here


    let s1 = gives_ownership();         // gives_ownership moves its return
    // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);

    println!("{:?}", s1);
    println!("{:?}", s2);
    println!("{:?}", s3);
    // println!("JSON data: {}", json_data); // This line would cause a compile-time error because json_data is moved
}


fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

let some_string = String::from("yours"); // some_string comes into scope

some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
             // scope

a_string  // a_string is returned and moves out to the calling function
}
