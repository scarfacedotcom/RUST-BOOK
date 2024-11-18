fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None // No result if division by zero
    } else {
        Some(a / b) // Returns the result as Some
    }
}

fn main() {
    let result = divide(10.0, 2.0); // Some(5.0)
    let error = divide(10.0, 0.0); // None

    println!("{:?}", result); // Output: Some(5.0)
    println!("{:?}", error); // Output: None
}
