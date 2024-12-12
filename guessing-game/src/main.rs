#![warn(clippy::all, clippy::pedantic)]

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess the number of roaches in the kitchen right now.");

    let secret_number: i32 = rand::thread_rng().gen_range(70..=100);

    loop {
        println!("how many roaches you think, boss?");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("something went wrong");

        let guess: i32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("please enter a number");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("there are more roaches than that."),
            Ordering::Greater => println!("there aren't that many... for now."),
            Ordering::Equal => {
                println!("Yea, there's actually that many. You better clean the kitchen!");
                break;
            }
        }
    }
}
