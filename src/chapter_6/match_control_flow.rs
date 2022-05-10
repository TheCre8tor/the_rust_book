enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

pub fn run() {
    let result = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("{}", result);

    // SECTION  2. Matching With Option ->
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let extract = extract_data(None);

    println!("With value: [{:?}], Without Value: [{:?}]", six, none);
    println!("Extracted Data: {}", extract);

    // SECTION  3.Catch-all Patterns and the _ Placeholder
    let dice_roll = 9;

    dice(dice_roll);

}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

// SECTION  2. Implemetation
fn plus_one(value: Option<i16>) -> Option<i16> {
    match value {
        None => None,
        Some(item) => Some(item + 1)
    }
}

fn extract_data(data: Option<i16>) -> i16 {
    match data {
        None => 0,
        Some(value) => value
    }
}

// SECTION  3.Catch-all Patterns and the _ Placeholder
fn dice(rolls: u8) {
    match rolls {
        3 => add_fancy_har(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
} 

fn add_fancy_har() {}

fn remove_fancy_hat() {}

fn move_player(num_spaces: u8) {
    println!("Moving {} spaces forward.", num_spaces);
}