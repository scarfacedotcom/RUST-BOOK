// struct Car {
//     owner: String,
//     year: u32,
//     fuel_level: f32,
//     price: u32,
// }

// fn display_car_info(car: &Car) {
//     println!(
//         "Owner: {}, Year: {}, Price: {}",
//         car.owner, car.year, car.price
//     );
// }

// fn main() {
//     let mut my_car = Car {
//         owner: String::from("ABC"),
//         year: 2000,
//         fuel_level: 0.00,
//         price: 5_000,
//     };
//     display_car_info(&my_car);
//     // println!("Hello, world!");
// }
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username,
//         email,
//         sign_in_count: 1,
//     }
// }

// fn main() {
//     let user1 = build_user(
//         String::from("someone@example.com"),
//         String::from("someusername123"),
//     );
// }
