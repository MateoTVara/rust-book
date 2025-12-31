fn main() {
    let number: i8 = 7;
    
    if number % 4 == 0 {
        divisible_by(4);
    } else if number % 3 == 0 {
        divisible_by(3);
    } else if number % 2 == 0 {
        divisible_by(2);
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if in a let statement
    let condition: bool = true;
    let conditionated_number = if condition { 5 } else { 6 };

    println!("\nThe value of number is: {conditionated_number}");
}

fn divisible_by(x: i8) {
    println!("number is divisible by {x}");
}
