fn main() {
    let lyrics: [&str; 12] = [
        "a Partridge in a Pear Tree",
        "Two Turtle Doves",
        "Three French Hens",
        "Four Calling Birds",
        "Five Gold Rings",
        "Six Geese a Laying",
        "Seven Swans a Swimming",
        "Eight Maids a Milking",
        "Nine Ladies Dancing",
        "Ten Lords a Leaping",
        "Eleven Pipers Piping",
        "Twelve Drummers Drumming",
    ];

    let ordinals: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth",
    ];

    for day in 0..12 {
        println!(
            "On the {} day of Christmas my true love sent to me:",
            ordinals[day]
        );
        for gift in (0..=day).rev() {
            if day > 0 && gift == 0 {
                println!("and {}.", lyrics[gift]);
            } else {
                println!("{},", lyrics[gift])
            }
        }

        println!();
    }
}
