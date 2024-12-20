// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

#[derive(Debug)]

struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }
}

fn main() {
    let rect1 = Rectangle {
        length: 20,
        width: 5,
    };

    println!("The area of reatangle is {} square meters", rect1.area())
}

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("rect1 is {rect1:?}");
// }
