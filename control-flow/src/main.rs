#[derive(Debug)]

// the code below should not be taken as a complete program, but rather disparate exercises, mostly unrelated.
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
    // match arms can bind to parts of the values that match the pattern.
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky Penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            // when Coin::Quarter matches, the state variable will bind to the value of that quarter's state.
            Coin::Quarter(state) => {
                println!("State quarter from {state:?}");
                25
            }
        }
    }
    value_in_cents(Coin::Quarter(UsState::Alaska));
    value_in_cents(Coin::Penny);

    // returning values inside Option<T>
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // catch all patterns
    let dice_roll = 9;
    match dice_roll {
        7 => add_fancy_socks(),
        9 => remove_fancy_socks(),
        _ => () // can use the unit type if no other code should run
        //  _ => reroll()
        // other => move_player(other),
    }
    fn add_fancy_socks() {}
    fn remove_fancy_socks() {}
    // fn reroll() {}
    // fn move_player(num_spaces: u8) {}

}


