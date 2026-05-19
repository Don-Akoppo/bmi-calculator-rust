pub struct BmiInput {
    pub weight: f64,
    pub height_cm: f64,
}

#[derive(Debug)]
pub enum BmiError {
    InvalidWeight,
    InvalidHeight,
}

pub fn calculate_bmi(data: BmiInput) -> Result<f64, BmiError> {
    if data.weight > 700.0 || data.weight < 10.0 {
        return Err(BmiError::InvalidWeight);
    }

    if data.height_cm > 290.0 || data.height_cm < 50.0 {
        return Err(BmiError::InvalidHeight);
    }

    let height_m = data.height_cm * 0.01;

    Ok(data.weight / (height_m * height_m))
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
