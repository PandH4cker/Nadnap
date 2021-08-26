use std::fs::read_to_string;
use std::iter::FromIterator;
use regex::Regex;

pub fn input_list_to_vec(path: &str) -> Vec<String> {
    if let Ok(str) = read_to_string(path) {
        let re = Regex::new(r"\r\n|\n").unwrap();
        return Vec::from_iter(re.split(&*str).map(String::from))
    }
    panic!("File {} doesn't exist", path)
}