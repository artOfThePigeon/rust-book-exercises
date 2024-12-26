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
            Coin::Nickel =>  5,
            Coin::Dime => 10,
            Coin::Quarter(_UsState) => 25,
        }
    }
}

fn main() {
    let mut tup: (u32, u32) = (0, 0);  // Just keep the tuple
    println!("Please insert a coin (Penny, Nickel, Dime, Quarter):");
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
                }
            },
            "q" => break,
            _ => {
                println!("Invalid coin!");
                continue;
            }
        };

        tup = count_coins(coin, tup);  // Just pass the tuple
        println!("Total coins: {}, Total value: ${:.2}", tup.0, (tup.1 as f64) / 100.0);  // Format cents as dollars
        println!("Please enter another coin or enter Q to quit.");
    }
}

fn count_coins(coin: Coin, mut tup: (u32, u32)) -> (u32, u32) {
    if let Coin::Quarter(state) = &coin {
        println!("State quarter from {:?}!", state);
    }
    tup.1 += coin.find_value();  // Add value to tup.1
    tup.0 += 1;                  // Increment count in tup.0
    tup
}