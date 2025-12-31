// fn main() {
//     let mut user1 = User {
//         active: true,
//         username: String::from("someusername123"),
//         email: String::from("someone@example.com"),
//         sign_in_count: 1,
//     };

//     user1.email = String::from("anotheremail@example.com");

//     // Create user2 by copying fields from user1
//     // let user2 = User {
//     //     active: user1.active,
//     //     username: user1.username,
//     //     email: String::from("another@example.com"),
//     //     sign_in_count: user1.sign_in_count,
//     // };

//     // Struct update syntax to copy remaining fields from user1
//     let user2 = User {
//         email: String::from("another@example.com"),
//         ..user1
//     };

//     // Ownership of user1.username is moved to user2.username since it doesn't
//     // implement the copy trait, the rest of user1's fields are still valid
//     // since user1.active and user1.sign_in_count do have the copy trait and
//     // user1.email was not used to create user2
// }

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         // username: username,
//         username,   // Field init shorthand
//         // email: email,
//         email,      // Field init shorthand
//         sign_in_count: 1,
//     }
// }















// #################
// # Tuple structs #
// #################

// // Tuple structs are similar to regular structs but
// // their fields do not have names associated with them.
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// fn main() {
//     // Although Color and Point are both tuple structs that
//     // take three i32 values, they are different types
//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);

//     // Access the fields using dot notation and indexes
//     println!("Black color RGB: ({}, {}, {})", black.0, black.1, black.2);

//     // Deestructure the tuple struct
//     println!(
//         "Origin point coordinates: ({}, {}, {})",
//         origin.0, origin.1, origin.2
//     );
//     let Point(x, y, z) = origin;
//     println!("Origin point coordinates: ({}, {}, {})", x, y, z);
// }















// #####################
// # Unit-like structs #
// #####################

struct ALwaysEqual;

fn main() {
    let subject = ALwaysEqual;
}