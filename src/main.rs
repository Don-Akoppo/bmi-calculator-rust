mod logic;

use logic::{calculate_bmi, get_category, BmiError, BmiInput};
use std::io::{self, Write}; // Импортируем io для работы с консолью

fn main() -> Result<(), BmiError> {
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

    let weight: u32 = match weight_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: Please enter a valid number for weight.");
            return Ok(());
        }
    };

    let height_cm: u32 = match height_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: Please enter a valid number for height.");
            return Ok(());
        }
    };

    let input = BmiInput { weight, height_cm };

    let bmi = calculate_bmi(input)?;
    let category = get_category(bmi);

    println!("\n---Result---");
    println!("Body Mass Index: {:.2}", bmi);
    println!("Your category: {:?}", category);
    
    Ok(())
}

