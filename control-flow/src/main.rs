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
}


