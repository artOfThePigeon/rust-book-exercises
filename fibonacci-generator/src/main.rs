use std::io;

fn main() {
    loop {
        println!("How many iterations of the fibonacci sequence do you want to calculate?");
        let iterations = get_iterations();
        println!(
            "The first {} iterations of the fibonacci sequence are:",
            iterations
        );

        fibonacci(iterations);

        if play_again() {
            continue;
        } else {
            break println!("Have a binacci day!");
        }
    }
}

fn get_iterations() -> u32 {
    loop {
        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("something went wrong.");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a non-negative integer.");
                continue;
            }
        };
        break input;
    }
}

fn fibonacci(n: u32) {
    let mut fib: u32 = 1;
    let mut fib_previous: u32 = 0;
    for _ in 1..=n {
        println!("{fib}");
        fib += fib_previous;
        fib_previous = fib - fib_previous;
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
