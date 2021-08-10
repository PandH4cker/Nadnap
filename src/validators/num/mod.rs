pub fn is_positive(v: &str) -> Result<(), String> {
    match v.parse::<u64>() {
        Ok(_) => { return Ok(()) }
        Err(_) => {
            if let Ok(f) = v.parse::<f64>() {
                if f > 0_f64 { return Ok(()) }
            }
        }
    }
    Err(format!("{} isn't a positive number", &*v))
}

pub fn is_number(v: &str) -> Result<(), String> {
    match v.parse::<f64>() {
        Ok(_) => { Ok(()) }
        Err(_) => {
            Err(format!("{} isn't a number", &*v))
        }
    }
}