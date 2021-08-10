use std::path::Path;

pub fn is_file(v: &str) -> Result<(), String> {
    match Path::new(v).is_file() {
        true => { Ok(()) }
        false => { Err(format!("{} isn't a valid file", v)) }
    }
}