extern crate rand;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("------------------");
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut counter = 0;

    loop {
        counter += 1;
        println!("attempt: {:?}", counter);
        println!("Type your guess!");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error to read entry");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You type: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Less!");
                println!("------------------");
            },
            Ordering::Greater => {
                println!("Greater!");
                println!("------------------");
            },
            Ordering::Equal => {
                println!("Right! You played {} times", counter);
                break;
            }
        }
    }
}
