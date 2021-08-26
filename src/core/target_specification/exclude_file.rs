use std::iter::FromIterator;
use std::fs::read_to_string;
use crate::core::target_specification::exclude_hosts::exclude_hosts;
use regex::Regex;

pub fn exclude_from_file(path: &str, t: &mut Vec<String>) {
    if let Ok(str) = read_to_string(path) {
        let re = Regex::new(r"\r\n|\n").unwrap();
        exclude_hosts(t, Vec::from_iter(re.split(&*str).map(String::from)))
    }
}