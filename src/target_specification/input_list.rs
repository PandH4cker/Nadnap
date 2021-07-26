use std::fs::read_to_string;
use std::iter::FromIterator;

pub fn input_list_to_vec(path: &str) -> Vec<String> {
    if let Ok(str) = read_to_string(path) {
        return Vec::from_iter(str.split("\n").map(String::from))
    }
    panic!("File {} doesn't exist", path)
}