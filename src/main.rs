use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    println!("Guess the number!");

    let mut guess: String = String::new();
    let secret_number: u32 = rand::thread_rng().gen_range(0..=100);

    println!("The secret number is: {}", secret_number);
    println!("Please, input a number: ");

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");


    println!("You guessed: {}", guess);

}
