use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to guessing game !!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Pleas input a number");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read");

        // parsing integer from String
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Your input was not a number");
                continue;
            }
        };
        // Using match to compare
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
