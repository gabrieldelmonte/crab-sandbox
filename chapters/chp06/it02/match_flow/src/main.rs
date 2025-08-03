#[derive(Debug)]
enum USState {
    Alabama
    // ...
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky you! You found a penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("This quarter is from {:#?}.", state);
            25
        },
    }
}

fn plus_one(value: Option<i32>) -> Option<i32> {
    match value {
        // Commenting out the line below will cause a compilation error!
        None => None,
        Some(number) => Some(number + 1),
    }
}

fn dice_roll(number: u8, option: u8) {
    if option == 1 {
        // Match function with 'other'
        let dice_roll_other = number;
        match dice_roll_other {
            1 => println!("1 - You rolled a one!"),
            2 => println!("1 - You rolled a two!"),
            3 => println!("1 - You rolled a three!"),
            4 => println!("1 - You rolled a four!"),
            5 => println!("1 - You rolled a five!"),
            6 => println!("1 - You rolled a six!"),
            _other => println!("1 - That's not a valid dice roll!"),
        }
    }
    else if option == 2 {
        // Match function with '_'
        let dice_roll_underscore = number;
        match dice_roll_underscore {
            1 => println!("2 - You rolled a one!"),
            2 => println!("2 - You rolled a two!"),
            3 => println!("2 - You rolled a three!"),
            4 => println!("2 - You rolled a four!"),
            5 => println!("2 - You rolled a five!"),
            6 => println!("2 - You rolled a six!"),
            _ => println!("2 - That's not a valid dice roll!"),
        }
    }
    else if option == 3 {
        // Match function with '_' but without returning a value
        let dice_roll_underscore_no_return = number;
        match dice_roll_underscore_no_return {
            1 => println!("3 - You rolled a one!"),
            2 => println!("3 - You rolled a two!"),
            3 => println!("3 - You rolled a three!"),
            4 => println!("3 - You rolled a four!"),
            5 => println!("3 - You rolled a five!"),
            6 => println!("3 - You rolled a six!"),
            _ => (),
        }
    }
    else {
        println!("Invalid option! Please choose 1, 2, or 3.");
    }
}

fn main() {
    let first_coin = Coin::Penny;
    let second_coin = Coin::Nickel;
    let third_coin = Coin::Dime;
    let fourth_coin = Coin::Quarter(USState::Alabama);
    let coins = vec![first_coin, second_coin, third_coin, fourth_coin];

    for coin in coins {
        println!("The value of {:?} coin is: {} cent(s)!", coin, value_in_cents(&coin));
    }
    println!();

    let number = Some(10);
    let second_number = plus_one(number);
    println!("The number ({:?}) plus one is: {:?}", number, second_number);

    let none_number: Option<i32> = None;
    let third_number = plus_one(none_number);
    println!("None number ({:?}) plus one is: {:?}", none_number, third_number);
    println!();

    let options = vec![1, 2, 3];
    let mut dice_roll_number = 1;
    for option in options {
        dice_roll(dice_roll_number, option);
        dice_roll_number *= 2;
    }
    println!();
}
