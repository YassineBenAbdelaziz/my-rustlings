use std::io;

fn main() {
    let mut choice = String::new();

    println!("Enter Your number");

    io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

    let choice :i32  = choice.trim().parse().expect("Failed to convert to number");

    if choice > 0 {
        println!("Positive !");
    } else if choice < 0 {
        println!("Negative Sir!");
    } else {
        println!("Null !");
    }
}
