pub struct BmiInput {
    pub weight: u32,
    pub height_cm: u32,
}

pub enum BmiError {
    InvalidWeight,
    InvalidHeight,
}
pub fn calculate_bmi(data: BmiInput) -> f64 {
    let weight_kg: f64 = data.weight as f64;
    let height_m: f64 = data.height_cm as f64 * 0.01;

    weight_kg / (height_m * height_m)
}
