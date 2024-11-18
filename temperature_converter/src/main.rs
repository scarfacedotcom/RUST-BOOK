use std::io;

fn main() {
    loop {
        println!("Simple Temperature Converter");
        println!("1. Convert Celsius to Fahrenheit");
        println!("2. Convert Fahrenheit to Celsius");
        println!("3. Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim() {
            "1" => {
                println!("Enter temperature in Celsius:");
                let celsius = read_number();
                let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
                println!("{:.2} °C is {:.2} °F", celsius, fahrenheit);
            }
            "2" => {
                println!("Enter temperature in Fahrenheit:");
                let fahrenheit = read_number();
                let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
                println!("{:.2} °F is {:.2} °C", fahrenheit, celsius);
            }
            "3" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please select 1, 2, or 3."),
        }
    }
}

fn read_number() -> f64 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().parse().unwrap_or(0.0)
}
