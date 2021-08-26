#![allow(non_snake_case, dead_code)]

use std::error::Error;

use crate::cli::generate_app;
use crate::core::target_specification;
use crate::core::target_specification::exclude_hosts::exclude_hosts;
use crate::core::target_specification::exclude_file::exclude_from_file;

mod validators;
mod constants;
mod cli;
mod core;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = generate_app().get_matches();
    let mut targets = vec![];
    if let Some(t) = matches.values_of(constants::args::target::NAME) {
        targets.append(&mut t.map(|s| s.to_string()).collect::<Vec<String>>())
    }
    if let Some(path) = matches.value_of(constants::args::input_list::NAME) {
        targets.append(&mut target_specification::input_list::input_list_to_vec(path))
    }
    if let Some(n) = matches.value_of(constants::args::input_random::NAME) {
        targets.append(
            &mut target_specification::input_random::generate_random_ips(
                n.parse::<u64>().unwrap()
            )
        )
    }
    if let Some(t) = matches.values_of(constants::args::exclude_hosts::NAME) {
        exclude_hosts(&mut targets, t.map(String::from).collect::<Vec<String>>())
    }
    if let Some(path) = matches.value_of(constants::args::exclude_file::NAME) {
        exclude_from_file(path, &mut targets)
    }

    println!("{:?}", targets);


    Ok(())
}
