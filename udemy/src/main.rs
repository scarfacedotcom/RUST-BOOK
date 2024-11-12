fn main() {
    // let x: i16 = 20.3;
    // printlin("x is: {y}")

    let x = 20;
    println!("x is: {x}");

    let y = 9;
    let y = y + 90;
    print!("t is : {y}");

    let v = 40;
    {
        let v = 45;
        println!("inner value is: {v}");
    }
    println!("outer value is: {v}");
}
