slint::include_modules!();

mod logic;
use logic::{calculate_bmi, get_category, BmiCategory, BmiError, BmiInput};
fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_weak = ui.as_weak();

    ui.on_calculate_bmi(move || {
        let ui = ui_weak.unwrap();

        let weight_str = ui.get_weight_text().to_string();
        let height_str = ui.get_height_text().to_string();

        let weight: f64 = match weight_str.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                ui.set_result_text("Error: Invalid weight format!".into());
                return;
            }
        };

        let height_cm: f64 = match height_str.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                ui.set_result_text("Error: Invalid height format!".into());
                return;
            }
        };

        let input = BmiInput { weight, height_cm };
        match calculate_bmi(input) {
            Ok(bmi) => {
                let category = get_category(bmi);

                let category_text = match category {
                    BmiCategory::SevereThinness => "Severe Thinness",
                    BmiCategory::ModerateThinness => "Moderate Thinness",
                    BmiCategory::MildThinness => "Mild Thinness",
                    BmiCategory::Normal => "Normal Weight",
                    BmiCategory::OverWeight => "Overweight",
                    BmiCategory::ObesityI => "Obese (Class I)",
                    BmiCategory::ObesityII => "Obese (Class II)",
                    BmiCategory::ObesityIII => "Obese (Class III)",
                };

                let message = format!("BMI: {:.1} ({})", bmi, category_text);
                ui.set_result_text(message.into());
            }
            Err(BmiError::InvalidWeight) => {
                ui.set_result_text("Error: Weight must be from 10 to 700 kg!".into());
            }
            Err(BmiError::InvalidHeight) => {
                ui.set_result_text("Error: Height must be from 50 to 290 cm!".into());
            }
        }
    });

    ui.run()
}
