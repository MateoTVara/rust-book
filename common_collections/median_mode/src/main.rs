use std::{io, collections::HashMap};

fn main() {
    println!("Provide a comman separated list of unsigned integers:");
    
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let str_list: Vec<&str> = input
        .trim()
        .split(",")
        .collect();

    let mut ui_list: Vec<u128> = Vec::new();

    for n in str_list {
        let n = n.trim();
        if n.is_empty() {continue};

        match n.parse::<u128>() {
            Ok(num) => ui_list.push(num),
            Err(_) => eprintln!("Invalid char found: {}", n),
        }
    }

    if ui_list.is_empty() {
        eprintln!("No valid numbers provided");
        return;
    }

    ui_list.sort();
    let len = ui_list.len();

    let median: f32 = if len % 2 == 0 {
        (ui_list[len / 2] as f32 + ui_list[len / 2 - 1] as f32) / 2.0
    } else {
        ui_list[(len - 1) / 2] as f32
    };

    let mut repeated: HashMap<u128, u128> = HashMap::new();
    for n in ui_list {
        let count = repeated.entry(n).or_insert(0);
        *count += 1;
    }

    let mut mode: i128 = -1;
    let mut mode_count = 0;
    for (value, count) in repeated {
        if count == mode_count {mode = -1}
        else if count > mode_count {
            mode = value as i128;
            mode_count = count;
        }
    }

    println!("Median is: {}", median);
    if mode == -1 {println!("Mode not found")} else {println!("Mode is: {}", mode)};

}
