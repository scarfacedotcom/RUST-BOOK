// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {result}");
// }

// fn main() {
//     'outer: loop {
//         println!("Simple Loop");
//         break 'outer;
//     }

//     let a = loop {
//         break 5;
//     };
// }

fn main() {
    let mut number = 3;
    while number != 0 {
        println! {"number has been reduced by 1, number  is now {number}"};
        number -= 1;
    }
    println! {"BUS-STOP"}
}
