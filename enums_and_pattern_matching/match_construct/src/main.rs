// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     // Quarter,
//     Quarter(UsState),
// }

// fn main() {
//     let coin = Coin::Quarter(UsState::Alaska);
//     let cents = value_in_cents(coin);
//     println!("The coin is worth {} cents.", cents);

//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);
//     println!("six: {:?}, none: {:?}", six, none);
// }

// // Function to get the value of a coin in cents
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         // Curly braces when you need to run multiples lines of code on an arm
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         },
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         // Pattern match with associated data
//         Coin::Quarter(state) => {
//             println!("State quarter from {state:?}!");
//             25
//         },
//     }
// }

// // Option<T> enum is defined in the standard library
// fn plus_one(x: Option<i32>) -> Option<i32> {
//     // Match on the Option<T> enum
//     match x {
//         // The arms pattern must cover all possible values
//         // It won't compile if any value is not covered
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }



















// ############################################
// # Catch-all patterns and the _ placeholder #
// ############################################
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // Catch-all pattern
        // other => move_player(other),
        // Using _ to ignore the value
        // _ => reroll(),
        // Unit value arm so that nothing happens
        _ => (),
    };
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}