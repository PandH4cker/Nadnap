#![allow(non_snake_case, dead_code)]

use crate::cli::generate_app;

mod validators;
mod constants;
mod cli;

fn main() {
    let matches = generate_app().get_matches();
    let targets : Vec<&str> = matches.values_of(constants::args::target::NAME)
                                     .unwrap()
                                     .collect::<Vec<&str>>();
    for t in targets {
        println!("{}", t);
    }

}
