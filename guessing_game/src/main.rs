use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number game!!!");

    let secret_number = rand::rng().random_range(1..=100);
    println!("secret number : {secret_number}");

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("please input a number");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too LESS..."),
            Ordering::Greater => println!("too BIG..."),
            Ordering::Equal => {
                println!("YOU WOOOOONNNN !!!! ");
                break;
            }
        }

        println!("you guessed {guess}");
    }
}
