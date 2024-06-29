use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    let secret = thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        println!("Please type your number: ");

        stdin()
            .read_line(&mut guess)
            .expect("Failed to catch input!");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
