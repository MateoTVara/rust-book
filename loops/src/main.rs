fn main() {
    // ###################################   
    // # Infinite loop with loop keyword #
    // ###################################
    // loop {
    //     println!("again");
    // }





    // ###############################
    // # Returning Values from Loops #
    // ###############################

    // let mut counter: i8 = 0;

    // let result: i8 = loop {
    //     counter += 1;
        
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("The result is {result}");





    // ###################################
    // # Disambiguating with Loop Labels #
    // ###################################

    // let mut count:i8 = 0;
    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }

    //         if count == 2 {
    //             break 'counting_up;
    //         }

    //         remaining -= 1;
    //     }

    //     count += 1;
    // }
    // println!("End count = {count}");




    // #############################################
    // # Streamlining Conditional Loops with while #
    // #############################################

    // let mut number: i8 = 3;

    // while number != 0 {
    //     println!("{number}");

    //     number -= 1;
    // }

    // println!("LIFTOFF!!!");





    // ########################################
    // # Looping Trough a Collection with for #
    // ########################################

    // let a: [i8; 5] = [10, 20, 30, 40, 50];
    
    // for element in a {
    //     println!("the value is: {element}");
    // }




    // ####################
    // # Loop using Range #
    // ####################

    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");

}












