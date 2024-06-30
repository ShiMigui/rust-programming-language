use std::io::stdin;

fn main() {
    let mut command = String::new();

    loop {
        println!();
        command.clear();
        println!("Temperature conversion!");
        println!("[C] - Convert Celsius to Fahrenheit");
        println!("[F] - Convert Fahrenheit to Celsius");
        println!("[Q] - Quit");
        println!("\nCommand:");

        match stdin().read_line(&mut command) {
            Ok(_) => {
                let trimmed = command.trim();

                if trimmed == "Q" {
                    break;
                } else if trimmed == "F" {
                    let _ = temperature_conversion(false, 'C');
                } else if trimmed == "C" {
                    let _ = temperature_conversion(true, 'F');
                } else {
                    eprintln!("Type a valid command! Please try again.");
                    
                    continue;
                }
            }
            Err(_) => {
                eprintln!("Failed to read line. Please try again.");
                continue;
            }
        };
    }
}

fn temperature_conversion(is_celsius_to_fahrenheit: bool, reverse_command: char) {
    println!("Enter temperature in {}:", if is_celsius_to_fahrenheit { "Celsius" } else { "Fahrenheit" });

    let mut value = String::new();

    match stdin().read_line(&mut value) {
        Ok(_) => {
            let input: f32 = match value.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("Invalid input. Please enter a valid number.");
                    return; // Exit function on error
                }
            };

            let result = if is_celsius_to_fahrenheit {
                celsius_fahrenheit(input)
            } else {
                fahrenheit_celsius(input)
            };

            println!("{} °{} is equals to {}", input, reverse_command, result);
        }
        Err(_) => {
            eprintln!("Failed to read line. Please try again.");
        }
    };
}

fn celsius_fahrenheit(celsius: f32) -> f32 {
    celsius * 1.8 + 32.0
}

fn fahrenheit_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) / 1.8
}
