#[derive(Debug)]
enum USState {
    Alabama,
    // ...
}

impl USState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            USState::Alabama => year >= 1819,
            // ...
        }
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}

fn describe_state_quarter(coin: &Coin) -> Option<String> {
    // Using 'let...else' to destructure the Coin enum and check for
    // Quarter variant and then checking the state of the quarter
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is kinda old!"))
    }
    else {
        Some(format!("{state:?} is not that old!"))
    }

    /*
    // Using 'if let' to destructure the Coin enum and check for
    // Quarter variant and then checking the state of the quarter
    let state = if let Coin::Quarter(state) = coin {
        state
    }
    else {
        return None;
    };
    
    if state.existed_in(1900) {
        Some(format!("{state:?} is kinda old!"))
    }
    else {
        Some(format!("{state:?} is not that old!"))
    }
    */

    // Alternatively, using a single 'if let' statement
    // to destructure the Coin enum and check for Quarter variant
    /*
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is kinda old!"))
        }
        else {
            Some(format!("{state:?} is not that old!"))
        }
    }
    else {
        None
    }
    */
}

fn main() {
    let config_max = Some(5u8);

    // Using 'match' to handle the Option type
    match config_max {
        Some(max) => println!("The maximum is set to {}!", max),
        _ => (),
    }

    // Using 'if let' to handle the Option type
    if let Some(max) = config_max {
        println!("The maximum is set to {}!", max);
    }
    println!();

    let mut count = 0;
    let first_coin = Coin::Penny;
    let second_coin = Coin::Nickel;
    let third_coin = Coin::Dime;
    let fourth_coin = Coin::Quarter(USState::Alabama);
    let coins = vec![first_coin, second_coin, third_coin, fourth_coin];
    for coin in &coins {
        if let Coin::Quarter(state) = coin {
            println!("State quarter from {:?}!", state);
        } else {
            count += 1;
        }
    }
    println!("There are {} non-quarter coins in a total of {} coins.", count, coins.len());
    println!();

    for coin in &coins {
        if let Some(description) = describe_state_quarter(&coin) {
            println!("{}", description);
        }
        else {
            println!("This is not a quarter! It is a {:?}!", coin);
        }
    }
    println!();
}
