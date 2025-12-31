// // Make a Rectangle struct and implement a function to calculate its area.
// // Also, derive the Debug trait for the Rectangle struct and print its instance.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}





// fn main() {

//     let scale = 2;

//     let rect1 = Rectangle {
//         // Using dbg! to inspect the value of scale during the calculation
//         // dbg macro takes ownership of the expression
//         // and returns ownership of the expression's value
//         width: dbg!(30 * scale),
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels",
//         area(&rect1)
//     );

//     // Debug derived trait is necessary for printing with {:?} or {:#?}
//     println!("{:#?}", rect1);

//     // We don't want dbg! to take ownership here, so I use a reference here
//     dbg!(&rect1);
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }















// Change the area function to be a method of the Rectangle struct.

impl Rectangle {

    // Method
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method
    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width > other.width && self.height > other.height) ||
        (self.width > other.height && self.height > other.width)
    }

    // Associated function (not a method)
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    let sq = Rectangle::square(40);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("Square created via a function associated to Rectangle struct: {:#?}", sq);
}

// multiple impl blocks are tottaly possible