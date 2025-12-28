// fn main() {
//     // String literal, always inmutable
//     // let mut s = "hello";
//     // Here, the variable is mutable so it can be reassigned other value
//     // s = "world";
//     // But the string literal is inmutable
//     // s.push_str("!") // X Invalid



//     // String type, can be mutated
//     // let mut s = String::from("hello");
//     // s.push_str(", world");
//     // println!("{}", s);



//     // Both are valid because integers have a know fixed size
//     // so both variables are assigned the same value
//     // let x = 5;
//     // let y = x;
//     // println!("{x}");



//     // Create a String and assign it to s1
//     // let s1 = String::from("hello");
//     // // Move ownership of the String from s1 to s2
//     // let s2 = s1;
//     // // s1 is no longer valid here; using s1 will cause a compile-time error
//     // println!("{s1}"); // ERROR: borrow of moved value: `s1`



//     // // To deeply copy the data from s1 to s2, we use the clone method
//     // let s1 = String::from("hello");
//     // let s2 = s1.clone();
//     // // Now both s1 and s2 are valid and own their own copies of the data
//     // println!("s1 = {}, s2 = {}", s1, s2);



//     // // s comes into scope
//     // let s = String::from("hello");
//     // takes_ownership(s); // s's value moves into the function...
//     // // ... and so is no longer valid here

//     // let x = 5; // x comes into scope
//     // makes_copy(x); // Because i32 implements the Copy trait,
//     // // x does NOT move into the function,
//     // // so it's okay to use x afterward.



//     // gives_ownership moves its return value into s1
//     let s1 = gives_ownership();

//     // s2 comes into scope
//     let s2 = String::from("hello");

//     // takes_and_gives_back moves s2 into
//     // and also moves its return value into s3
//     let s3 = takes_and_gives_back(s2);
// }

// fn takes_ownership(some_string: String) { // some_string comes into scope
//     println!("{some_string}");
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.

// fn makes_copy(some_integer: i32) { // some_integer comes into scope
//     println!("{some_integer}");
// } // Here, some_integer goes out of scope. Nothing special happens.



// fn gives_ownership() -> String {       // gives_ownership will move its
//                                        // return value into the function
//                                        // that calls it

//     let some_string = String::from("yours"); // some_string comes into scope

//     some_string                        // some_string is returned and
//                                        // moves out to the calling
//                                        // function
// }

// // This function takes a String and returns a String.
// fn takes_and_gives_back(a_string: String) -> String {
//     // a_string comes into
//     // scope

//     a_string  // a_string is returned and moves out to the calling function
// }















// ############################
// # References and borrowing #
// ############################

// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// // This function takes a reference to a String as a parameter
// // and returns its length. The reference is indicated by the & symbol.
// // A reference allows you to refer to some value without taking ownership of it.
// // The act of creating a reference is called borrowing.
// fn calculate_length(s: &String) -> usize {
//     s.len()
// }



// What happens when we try to modify a borrowed value?
// fn main() {
//     let s = String::from("hello");

//     change(&s);
// }

// fn change(s: &String) {
//     // Just as variables are immutable by default,
//     // references are also immutable by default.
//     s.push_str(", world");
// }



// Mutable references
// fn main() {
//     let mut s = String::from("hello");

//     // It's possible to create inmutable references to a mutable variable.
//     // But not the other way around.
//     change(&mut s);

//     println!("{}", s);
// }

// fn change(some_string: &mut String) {
//     // Mutable references allow you to change the value they refer to.
//     some_string.push_str(", world");
// }



// Biggest restriction with mutable references
// fn main() {
//     let mut s = String::from("hello");

//     // You can have only one mutable reference to a particular piece of data
//     // So it can be said that you can only borrow it mutably once at a time.
//     let r1 = &mut s;
//     let r2 = &mut s;

//     println!("{}, {}", r1, r2);
// }



// To prevent dataaces, Rust enforces a rule that you can have either
// one mutable reference or any number of inmutable references to a
// particular piece of data in a particular scope, but not both at the same time.
// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     let r3 = &mut s; // BIG PROBLEM
//     println!("{}, {}, {}", r1, r2, r3);



//     // let r1 = &s; // no problem
//     // let r2 = &s; // no problem
//     // // variables r1 and r2 will not be used after this point
//     // let r3 = &mut s; // no problem
//     // println!("{}", r3);
// }



// Dangling references
fn main() {
    let reference_to_nothing = dangle();
}

// This function tries to return a reference to a String,
// but the String will go out of scope when the function
// ends, so the reference would be pointing to invalid data.
fn dangle() -> 
    // &String
    String
{
    let s = String::from("hello");

    // If you try to return a reference to s here,
    // it would be a dangling reference.
    // &s

    // To fix this, we return the String itself, not a reference to it.
    // Ownership is moved out of the function, and nothing is deallocated.
    s
}


