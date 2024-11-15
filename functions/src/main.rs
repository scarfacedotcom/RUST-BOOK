fn main() {
    println!("Hello, world!");
    my_fn("This is my function");
    let str = "Function call with a variable";
    my_fn(str);
}

fn my_fn(s: &str) {
    println!("{s}");
}
