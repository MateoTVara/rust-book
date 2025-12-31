use std::io;

fn main() {
    'app: loop {
        println!("Enter a unsigned integer number");

        let mut unsigned_integer = String::new();

        io::stdin()
            .read_line(&mut unsigned_integer)
            .expect("Failed to read line");

        let unsigned_integer: u128 = match unsigned_integer.trim().parse() {
            Ok(num) => {
                if num == 0 {
                    println!("Not valid number\n");
                    continue 'app;
                }
                num
            },
            Err(_) => {
                println!("Not valid number\n");
                continue 'app;
            },
        };

        if unsigned_integer > 187 {
            println!("Unsopported fibo number\n");
            continue 'app;  
        }

        if unsigned_integer == 1 {
            println!("Is: 0\n");
            continue 'app;
        } 
        
        if unsigned_integer == 2 {
            println!("Is: 1\n");
            continue 'app;
        }

        let mut acc: u128 = 0;
        let mut n = 3;
        let [mut last, mut penul]: [u128; 2] = [1, 0];

        while n <= unsigned_integer {
            acc = last + penul;
            penul = last;
            last = acc;
            n += 1;
        }

        println!("Is: {acc}!\n");

    }
}
