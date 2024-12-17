#![warn(clippy::all, clippy::pedantic)]
use std::io;

fn main() {
    loop {
        println!("How many iterations of the fibonacci sequence do you want to calculate?");
        let n: u32 = get_iterations();
        println!("The first {n} iterations of the fibonacci sequence are:");
        fibonacci(n);

        if play_again() {
            continue;
        }
        break println!("Have a binacci day!");
    }
}

fn get_iterations() -> u32 {
    loop {
        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("something went wrong.");

        let input: u32 = if let Ok(num) = input.trim().parse() {
            num
        } else {
            println!("Please enter a non-negative integer.");
            continue;
        };
        break input;
    }
}

fn fibonacci(n: u32) {
    let mut fib: u128 = 1;
    let mut fib_previous: u128 = 0;
    for i in 1..=n {
        println!("{fib}");
        if let Some(next_fib) = fib.checked_add(fib_previous) {
            fib = next_fib;
            fib_previous = fib - fib_previous;
        } else {
            println!("Overflow occurred at iteration {i}. This is as large as it goes!");
            return;
        }
    }
}

fn play_again() -> bool {
    println!("would you like to try again? please enter Y or N.");
    loop {
        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("something went wrong");

        match input.trim().to_uppercase().as_str() {
            "Y" => return true,
            "N" => return false,
            _ => {
                println!("please enter Y or N.");
                continue;
            }
        }
    }
}
