use std::io;

fn main() {
    println!("Select the option in the menu");

    'menu: loop {

        println!("1.- Farenheit to Celcius");
        println!("2.- Celcius to Farenheit");
        println!("3.- Exit program");

        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let option: u8 = match option.trim().parse() {
            Ok(num) => {
                if num == 3 {
                    println!("Exiting!");
                    break 'menu;
                } else if num > 2  {
                    println!("Invalidad menu option\n");
                    continue 'menu;
                }
                num
            },
            Err(_) => {
                println!("Invalid menu option\n");
                continue 'menu;
            },
        };

        'convertion: loop {

            println!("Enter the temperature:");

            let mut temperature = String::new();

            io::stdin()
                .read_line(&mut temperature)
                .expect("Faile to read line");

            let temperature: i32 = match temperature.trim().parse() {
                Ok(num) => {
                    if option == 1 && num < -459 {
                        println!("Below minimun farenheit possible\n");
                        continue 'convertion;
                    } else if option == 2 && num < -273 {
                        println!("Below minimun celcius possible\n");
                        continue 'convertion;
                    }
                    num
                },
                Err(_) => {
                    println!("Invalidad temperature\n");
                    continue 'convertion;
                }
            };

            let to_return: i32;

            if option == 1 {        // to celcius
                to_return = (temperature - 32) * 5 / 9;
            } else {                // to farenheit
                to_return = temperature * 9 / 5 + 32;
            }

            println!("Converted temperature: {to_return}\n");
            break 'convertion;
        }

    }
}
