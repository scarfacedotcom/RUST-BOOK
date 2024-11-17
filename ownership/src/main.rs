//fn main() {
// let mut s = String::from("hello");
// s.push_str(", world");
// println!("{s}");

// let x = 5;
// let y = x;
// println!("x is {x}");
// println!("y is {y}");

// let s1 = String::from("hello");
// let s2 = s1;
// println!("s1 is {s1}");
// println!("s2 is {s2}");

// let s1 = String::from("hello");
// let s2 = s1.clone();
// println!("{s2}, world, {s1} wworld");
//}

// fn main() {
//     let s = String::from("hello"); // s comes into scope

//     takes_ownership(s); // s's value moves into the function...
//                         // ... and so is no longer valid here

//     let x = 5; // x comes into scope

//     makes_copy(x); // x would move into the function,
//                    // but i32 is Copy, so it's okay to still
//                    // use x afterward
// } // Here, x goes out of scope, then s. But because s's value was moved, nothing
//   // special happens.

// fn takes_ownership(some_string: String) {
//     // some_string comes into scope
//     println!("{some_string}");
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.

// fn makes_copy(some_integer: i32) {
//     // some_integer comes into scope
//     println!("{some_integer}");
// } // Here, some_integer goes out of scope. Nothing special happens.

fn main() {
    let vec_1 = vec![1, 2, 3];
    takes_ownership(&vec_1);
    println!("vec is: {:?}", vec_1);
}

fn takes_ownership(vec: Vec<i32>) {
    println!("vec is: {:?}", vec);
}
