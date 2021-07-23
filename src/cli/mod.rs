use clap::{
    crate_name,
    crate_version,
    crate_authors,
    crate_description,
    App,
    Arg
};
use crate::constants::{
    USAGE_STR,
    args::*,
    groups::*
};
use crate::validators;

pub fn generate_app() -> App<'static, 'static> {
    App::new(crate_name!())
        .usage(USAGE_STR)
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())

        .arg(
            Arg::with_name(input_list::NAME)
                .long(input_list::LONG)
                .value_name(input_list::VALUE_NAME)
                .help(input_list::HELP)
                .takes_value(true)
                .group(target_specification::NAME)
                .number_of_values(input_list::NUMBER_OF_VALUES)
                .validator(validators::fs::is_file)
        )
        .arg(
            Arg::with_name(input_random::NAME)
                .long(input_random::LONG)
                .value_name(input_random::VALUE_NAME)
                .help(input_random::HELP)
                .takes_value(true)
                .group(target_specification::NAME)
                .number_of_values(input_random::NUMBER_OF_VALUES)
                .validator(validators::num::is_positive)
        )
        .arg(
            Arg::with_name(exclude_hosts::NAME)
                .long(exclude_hosts::LONG)
                .value_name(exclude_hosts::VALUE_NAME)
                .help(exclude_hosts::HELP)
                .takes_value(true)
                .group(target_specification::NAME)
        )
}