#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let config_max = Some(3u8);

    // Annoying boilerplate to handle an optional value with just one case
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {}", max),
    //     None => (),
    // }

    // Improved version using if let
    // The code will only execute the block if config_max is Some
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }





    // if let else example
    let config_min: Option<u8> = None;
    if let Some(min) = config_min {
        println!("The minimum is configured to be {}", min);
    } else {
        println!("No minimum is configured.");
    }

    // The previous code is equivalent to the following match statement
    match config_min {
        Some(min) => println!("The minimum is configured to be {}", min),
        None => println!("No minimum is configured."),
    }
}


fn describe_state_quarter(coin: Coin) -> Option<String> {
    // Using let else to extract the state from a Quarter coin
    // If the coin is not a Quarter, return None
    let Coin::Quarter(state) = coin else {
        return None;
    };

    // If it is a Quarter, return Some description
    Some(format!("State quarter from {:?}", state))
}
