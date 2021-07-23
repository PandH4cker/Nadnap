use std::path::Path;

pub fn is_file(v: String) -> Result<(), String> {
    match Path::new(v.as_str()).is_file() {
        true => { Ok(()) }
        false => { Err(format!("{} isn't a valid file", v)) }
    }
}