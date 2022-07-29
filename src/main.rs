use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to guessing game !!");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    println!("Pleas input a number");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read");

    println!("You have guessed: {guess}");

    // parsing integer from String
    let guess: u32 = guess.trim().parse().expect("Please insert a number");
    // Using match to compare
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too Small"),
        Ordering::Greater => println!("Too Big"),
        Ordering::Equal => println!("You win"),
    }
}
