use std::io;

fn convert_temperature(value: f64, to_celsius: bool) -> f64 {
    if to_celsius {
        (value - 32.0) * 5.0 / 9.0 // Fahrenheit to Celsius
    } else {
        value * 9.0 / 5.0 + 32.0 // Celsius to Fahrenheit
    }
}

fn main() {
    loop {
        println!("Temperature Converter");
        println!("1. Convert Celsius to Fahrenheit");
        println!("2. Convert Fahrenheit to Celsius");
        println!("3. Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim() {
            "1" => {
                let value = get_temperature_input("Celsius");
                let result = convert_temperature(value, false);
                println!("{:.2} °C is equal to {:.2} °F", value, result);
            }
            "2" => {
                let value = get_temperature_input("Fahrenheit");
                let result = convert_temperature(value, true);
                println!("{:.2} °F is equal to {:.2} °C", value, result);
            }
            "3" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please select 1, 2, or 3."),
        }
    }
}

fn get_temperature_input(unit: &str) -> f64 {
    println!("Enter the temperature in {}:", unit);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            0.0 // Default value in case of error
        }
    }
}
