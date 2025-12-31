fn main() {
    println!("Hello, world!");
    another_function(32, 'h');

    let five: i32 = five();
    println!("{five}");
    let plus_one = plus_one(five);
    println!("five value plust one: {plus_one}");
}

fn another_function(x: i32, unit_label: char) {
    println!("The measurement is: {x}{unit_label}");
}

// Functions with return values
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}