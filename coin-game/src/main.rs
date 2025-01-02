use std::io::stdin;
use std::str::FromStr;
use strum_macros::{EnumString, Display}; // adds the ability to convert strings into enum variants, and vice versa

#[derive(Debug, EnumString, Display)] // automatically implements these traits for my enum, giving it the ability to convert between strings and enum variants.
#[strum(ascii_case_insensitive)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    WestVirginia,
    Wisconsin,
    Wyoming,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn find_value(&self) -> u32 {
        match &self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(_UsState) => 25,
        }
    }

    // New helper function to get coin name
    fn get_name(&self) -> String {
        match self {
            Coin::Penny => "Penny".to_string(),
            Coin::Nickel => "Nickel".to_string(),
            Coin::Dime => "Dime".to_string(),
            Coin::Quarter(state) => format!("Quarter ({})", state),
        }
    }
}

struct CoinCollection {
    count: u32,
    value: u32,
    penny_count: u32,    // New counters for each coin type
    nickel_count: u32,
    dime_count: u32,
    quarter_count: u32,
}

fn main() {
    let mut collection = CoinCollection {
        count: 0,
        value: 0,
        penny_count: 0,
        nickel_count: 0,
        dime_count: 0,
        quarter_count: 0,
    };

    println!("Welcome to Coin Collector!");
    println!("Commands: Penny, Nickel, Dime, Quarter, Status, Q (quit)");

    loop {
        let mut entry = String::new();
        stdin()
            .read_line(&mut entry)
            .expect("Failed to read line");

        let coin = match entry.trim().to_lowercase().as_str() {
            "penny" => Coin::Penny,
            "nickel" => Coin::Nickel,
            "dime" => Coin::Dime,
            "quarter" => {
                println!("Ooooh what state?");
                loop {
                    let mut state = String::new();
                    stdin()
                        .read_line(&mut state)
                        .expect("error");

                    if let Ok(state) = UsState::from_str(state.trim()) {
                        break Coin::Quarter(state)
                    }
                    println!("Invalid state! Please try again.");
                }
            },
            "status" => {
                print_status(&collection);
                continue;
            },
            "q" => break,
            _ => {
                println!("Invalid command! Try: Penny, Nickel, Dime, Quarter, Status, or Q");
                continue;
            }
        };

        collection = count_coins(coin, collection);
        println!("Total coins: {}, Total value: ${:.2}", collection.count, (collection.value as f64) / 100.0);
        println!("Please enter another coin or enter Status for detailed view, Q to quit.");
    }

    // Print final status when quitting
    println!("\nFinal Collection Status:");
    print_status(&collection);
}

fn count_coins(coin: Coin, mut collection: CoinCollection) -> CoinCollection {
    if let Coin::Quarter(state) = &coin {
        println!("State quarter from {:?}!", state);
    }

    // Update specific coin counters
    match &coin {
        Coin::Penny => collection.penny_count += 1,
        Coin::Nickel => collection.nickel_count += 1,
        Coin::Dime => collection.dime_count += 1,
        Coin::Quarter(_) => collection.quarter_count += 1,
    }

    collection.value += coin.find_value();
    collection.count += 1;
    collection
}

// New function to print detailed status
fn print_status(collection: &CoinCollection) {
    println!("\n=== Collection Status ===");
    println!("Pennies: {}", collection.penny_count);
    println!("Nickels: {}", collection.nickel_count);
    println!("Dimes: {}", collection.dime_count);
    println!("Quarters: {}", collection.quarter_count);
    println!("Total coins: {}", collection.count);
    println!("Total value: ${:.2}", (collection.value as f64) / 100.0);
    println!("=====================\n");
}