mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn server_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

fn deliver_order() {}

// Bringing the hosting module into scope
// pub keyword makes it accessible from outside this module
// effectively re-exporting it
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    // front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // This line would cause a compile-time error
    // because seasonal_fruit is private
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // Using the hosting module brought into scope
    hosting::add_to_waitlist();
}






// ########################################
// # Disambiguating between similar names #
// ########################################

// use std::fmt;
// use std::io;

// fn function1() -> fmt::Result {
//     // --snip--
//     Ok(())
// }
// fn function2() -> io::Result<()> {
//     // --snip--
//     Ok(())
// }

// Using 'as' to create an alias
// use std::fmt::Result as FmtResult;
// use std::io::Result as IoResult;





// ################
// # Nested Paths #
// ################

// These are useful when importing multiple items from the same module

// use std::{cmp::Ordering, io};
// use std::io::{self, Write};




// ########
// # Glob #
// ########
// use std::collections::*;