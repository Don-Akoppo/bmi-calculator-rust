mod logic;

use logic::{BmiError, BmiInput, calculate_bmi, get_category};

fn main() -> Result<(), BmiError> {
    let input = BmiInput {
        weight: 75,
        height_cm: 180,
    };

    let bmi = calculate_bmi(input)?;
    let category = get_category(bmi);

    println!("---Result---");
    println!("Body Mass Index: {:.2}", bmi);
    println!("Your category: {:?}", category);
    Ok(())
}
