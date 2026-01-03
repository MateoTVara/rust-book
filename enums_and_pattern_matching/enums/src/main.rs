// // Define an enum to represent an IP address, which can be either IPv4 or IPv6
// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// // Define an enum to represent different types of messages
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// // The Message enum is the same as defining separate structs for each message type
// struct QuitMessage;
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String);  // Tuple struct
// struct ChangeColorMessage(i32, i32, i32);  // Tuple struct


// // It's possible to implement methods on enums
// impl Message {
//     fn call(&self) {
//         // method body would go here
//     }

//     fn some_associated_function() {
//         // function body would go here
//     }
// }

// fn main() {
//     let four: IpAddr = IpAddr::V4(127, 0, 0, 1);
//     let six: IpAddr = IpAddr::V6(String::from("::1"));

//     let m = Message::Write(String::from("hello"));
//     m.call();
//     Message::some_associated_function();
// }





















// The option enum is defined by the standard library as follows:
// enum Option<T> {
//     None,
//     Some(T),
// }
// It's not necessary to bring Option into scope explicitly because it's included in the prelude
// Since null values are not allowed in Rust, Option is used to encode the scenario where a value could be something or nothing
fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    // It's not possible to operate directly on the values inside an Option without first handling the None case
    // For example, the following line would cause a compile-time error:
    // let sum = some_number + 5; // Error!
}
