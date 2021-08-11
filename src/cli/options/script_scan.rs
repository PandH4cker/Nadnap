use clap::{
    App, Arg, ArgGroup
};
use crate::constants::{args::*, groups::*};
//use crate::validators;

pub fn add_script_scan(app: App<'static>) -> App<'static> {
    app
        .help_heading(script_scan::NAME)
        .arg(
            Arg::new(default_script::NAME)
                .about(default_script::HELP)
                .long(default_script::LONG)
                .takes_value(false)
        )
        .arg(
            Arg::new(script::NAME)
                .about(script::HELP)
                .long(script::LONG)
                .takes_value(true)
                .value_name(script::VALUE_NAME)
                .require_equals(true)
                .multiple(true)
                .value_delimiter(script::VALUE_DELIMITER)
        )
        .arg(
            Arg::new(script_args::NAME)
                .about(script_args::HELP)
                .long(script_args::LONG)
                .takes_value(true)
                .value_name(script_args::VALUE_NAME)
                .require_equals(true)
                .multiple(true)
                .value_delimiter(script_args::VALUE_DELIMITER)
        )
        .arg(
            Arg::new(script_args_file::NAME)
                .about(script_args_file::HELP)
                .long(script_args_file::LONG)
                .takes_value(true)
                .value_name(script_args_file::VALUE_NAME)
                .require_equals(true)
        )
        .arg(
            Arg::new(script_trace::NAME)
                .about(script_trace::HELP)
                .long(script_trace::LONG)
                .takes_value(false)
        )
        .arg(
            Arg::new(script_update_db::NAME)
                .about(script_update_db::HELP)
                .long(script_update_db::LONG)
                .takes_value(false)
        )
        .arg(
            Arg::new(script_help::NAME)
                .about(script_help::HELP)
                .long(script_help::LONG)
                .takes_value(true)
                .value_name(script_help::VALUE_NAME)
                .require_equals(true)
                .multiple(true)
                .value_delimiter(script_help::VALUE_DELIMITER)
        )
        .group(
            ArgGroup::new(script_scan::NAME)
                .args(&[
                    default_script::NAME, script::NAME, script_args::NAME,
                    script_args_file::NAME, script_trace::NAME, script_update_db::NAME,
                    script_help::NAME
                ])
                .multiple(true)
        )
}