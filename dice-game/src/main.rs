use std::io::stdin;

#[derive(Debug)]
enum UsState {
    Alaska,
    Alabama,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let mut count = 0;  // Keep track of total coins

    println!("Please insert a coin (Penny, Nickel, Dime, Quarter):");
    let mut entry = String::new();
    stdin()
        .read_line(&mut entry)
        .expect("Failed to read line");

    let coin = match entry.trim() {
        "Penny" => Coin::Penny,
        "Nickel" => Coin::Nickel,
        "Dime" => Coin::Dime,
        "Quarter" => Coin::Quarter(UsState::Alaska),  // Default to Alaska for now
        _ => {
            println!("Invalid coin!");
            return;
        }
    };

    count = count_coins(coin);
    println!("Total coins: {}", count);
}

fn count_coins(coin: Coin) -> u32 {
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
        count += 1;
    } else {
        count += 1;
    }
    count
}