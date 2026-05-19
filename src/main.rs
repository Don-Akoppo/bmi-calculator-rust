mod logic;

use logic::{BmiInput, calculate_bmi, get_category};
use std::io::{self, Write};

fn main() {
    let mut weight_str = String::new();
    let mut height_str = String::new();

    print!("Enter your weight (kg): ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut weight_str)
        .expect("Failed to read line");

    print!("Enter your height (cm): ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut height_str)
        .expect("Failed to read line");

    let weight: f64 = match weight_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Please enter a valid number for weight.");
            return;
        }
    };

    let height_cm: f64 = match height_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Please enter a valid number for height.");
            return;
        }
    };

    let input = BmiInput { weight, height_cm };

    match calculate_bmi(input) {
        Ok(bmi) => {
            let category = get_category(bmi);
            println!("\n--- Result ---");
            println!("Body Mass Index: {:.2}", bmi);
            println!("Your category: {:?}", category);
        }
        Err(e) => {
            eprintln!("\nError: Validation failed. {:?}", e);
        }
    }
}
