pub struct BmiInput {
    pub weight: u32,
    pub height_cm: u32,
}
#[derive(Debug)]
pub enum BmiError {
    InvalidWeight,
    InvalidHeight,
}
pub fn calculate_bmi(data: BmiInput) -> Result<f64, BmiError> {
    if data.weight > 700 || data.weight < 10 {
        return Err(BmiError::InvalidWeight);
    }

    if data.height_cm > 290 || data.height_cm < 50 {
        return Err(BmiError::InvalidHeight);
    }

    let weight_kg: f64 = data.weight as f64;
    let height_m: f64 = data.height_cm as f64 * 0.01;

    Ok(weight_kg / (height_m * height_m))
}
