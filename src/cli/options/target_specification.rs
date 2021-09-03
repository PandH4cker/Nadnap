use clap::{
    App, Arg, ArgGroup, ValueHint
};
use crate::constants::{args::*, groups::*};
use crate::validators;

pub fn add_target_specification(app: App<'static>) -> App<'static> {
    app
        .help_heading(target_specification::NAME)
        .arg(
            Arg::new(target::NAME)
                .last(true)
                .takes_value(true)
                .value_delimiter(target::VALUE_DELIMITER)
                .multiple(true)
                .validator(validators::net::is_hosts)
                .required_unless_present_any(&[input_list::NAME, input_random::NAME])
                .value_hint(ValueHint::Hostname)
        )
        .arg(
            Arg::new(input_list::NAME)
                .long(input_list::LONG)
                .value_name(input_list::VALUE_NAME)
                .about(input_list::HELP)
                .takes_value(true)
                .number_of_values(input_list::NUMBER_OF_VALUES)
                .validator(validators::fs::is_file)
                .required_unless_present_any(&[target::NAME, input_random::NAME])
                .value_hint(ValueHint::FilePath)
        )
        .arg(
            Arg::new(input_random::NAME)
                .long(input_random::LONG)
                .value_name(input_random::VALUE_NAME)
                .about(input_random::HELP)
                .takes_value(true)
                .number_of_values(input_random::NUMBER_OF_VALUES)
                .validator(validators::num::is_positive)
                .required_unless_present_any(&[input_list::NAME, target::NAME])
                .value_hint(ValueHint::Other)
        )
        .arg(
            Arg::new(exclude_hosts::NAME)
                .long(exclude_hosts::LONG)
                .value_name(exclude_hosts::VALUE_NAME)
                .about(exclude_hosts::HELP)
                .takes_value(true)
                .validator(validators::net::is_hosts)
                .use_delimiter(true)
                .value_hint(ValueHint::Hostname)
                .multiple(true)
        )
        .arg(
            Arg::new(exclude_file::NAME)
                .long(exclude_file::LONG)
                .value_name(exclude_file::VALUE_NAME)
                .about(exclude_file::HELP)
                .takes_value(true)
                .validator(validators::fs::is_file)
                .number_of_values(exclude_file::NUMBER_OF_VALUES)
                .value_hint(ValueHint::FilePath)
        )
        .group(
            ArgGroup::new(target_specification::NAME)
                .args(&[
                    target::NAME, input_list::NAME, input_random::NAME,
                    exclude_hosts::NAME, exclude_file::NAME
                ])
                .multiple(true)
        )
}