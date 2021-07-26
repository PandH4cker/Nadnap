#![allow(non_snake_case, dead_code)]

use crate::cli::generate_app;

mod validators;
mod constants;
mod cli;

fn main() {
    let _matches = generate_app().get_matches();

}
