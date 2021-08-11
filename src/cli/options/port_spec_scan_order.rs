use clap::{
    App, Arg, ArgGroup, ValueHint
};
use crate::constants::{args::*, groups::*};

pub fn add_port_spec_scan_order(app: App<'static>) -> App<'static> {
    app
        .help_heading(port_specification_scan_order::NAME)
        .arg(
            Arg::new(port_ranges::NAME)
                .short(port_ranges::SHORT)
                .about(port_ranges::HELP)
                .takes_value(true)
                .value_name(port_ranges::VALUE_NAME)
                .value_hint(ValueHint::Other)
        )
        .arg(
            Arg::new(exclude_ports::NAME)
                .long(exclude_ports::LONG)
                .about(exclude_ports::HELP)
                .takes_value(true)
                .value_name(exclude_ports::VALUE_NAME)
                .value_hint(ValueHint::Other)
        )
        .arg(
            Arg::new(fast_mode::NAME)
                .short(fast_mode::SHORT)
                .about(fast_mode::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::new(dont_randomize::NAME)
                .short(dont_randomize::SHORT)
                .about(dont_randomize::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::new(top_ports::NAME)
                .long(top_ports::LONG)
                .about(top_ports::HELP)
                .takes_value(true)
                .value_name(top_ports::VALUE_NAME)
                .value_hint(ValueHint::Other)
        )
        .arg(
            Arg::new(port_ratio::NAME)
                .long(port_ratio::LONG)
                .about(port_ratio::HELP)
                .takes_value(true)
                .value_name(port_ratio::VALUE_NAME)
                .value_hint(ValueHint::Other)
        )
        .group(
            ArgGroup::new(port_specification_scan_order::NAME)
                .args(&[
                    port_ranges::NAME, exclude_ports::NAME, fast_mode::NAME,
                    dont_randomize::NAME, top_ports::NAME, port_ratio::NAME
                ])
                .multiple(true)
        )
}