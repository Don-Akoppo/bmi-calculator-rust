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
#[derive(Debug)]
pub enum BmiCategory {
    SevereThinness,
    ModerateThinness,
    MildThinness,
    Normal,
    OverWeight,
    ObesityI,
    ObesityII,
    ObesityIII,
}
pub fn get_category(bmi: f64) -> BmiCategory {
    if bmi < 16.0 {
        BmiCategory::SevereThinness
    } else if bmi < 17.0 {
        BmiCategory::ModerateThinness
    } else if bmi < 18.5 {
        BmiCategory::MildThinness
    } else if bmi < 25.0 {
        BmiCategory::Normal
    } else if bmi < 30.0 {
        BmiCategory::OverWeight
    } else if bmi < 35.0 {
        BmiCategory::ObesityI
    } else if bmi < 40.0 {
        BmiCategory::ObesityII
    } else {
        BmiCategory::ObesityIII
    }
}
