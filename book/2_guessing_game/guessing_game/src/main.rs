use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Guessing Game :D");

    let random_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("Enter the guessed number:");
        
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to red line");

        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&random_number) {
            Ordering::Greater => println!("Too big !"),
            Ordering::Less => println!("Too small !"),
            Ordering::Equal => {
                println!("You guessed right ! You WON !");
                break;
            }
        }
    }
}