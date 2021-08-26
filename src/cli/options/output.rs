use clap::{
    App, Arg, ArgGroup
};
use crate::constants::{args::*, groups::*};

pub fn add_output(app: App<'static>) -> App<'static> {
    app
        .help_heading(output::NAME)
        .arg(
            Arg::new(normal_output::NAME)
                .long(normal_output::LONG)
                .about(normal_output::HELP)
                .takes_value(true)
                .value_name(normal_output::VALUE_NAME)
        )
        .arg(
            Arg::new(XML_output::NAME)
                .long(XML_output::LONG)
                .about(XML_output::HELP)
                .takes_value(true)
                .value_name(XML_output::VALUE_NAME)
        )
        .arg(
            Arg::new(script_kiddie_output::NAME)
                .long(script_kiddie_output::LONG)
                .about(script_kiddie_output::HELP)
                .takes_value(true)
                .value_name(script_kiddie_output::VALUE_NAME)
        )
        .arg(
            Arg::new(grepable_output::NAME)
                .long(grepable_output::LONG)
                .about(grepable_output::HELP)
                .takes_value(true)
                .value_name(grepable_output::VALUE_NAME)
        )
        .arg(
            Arg::new(all_output::NAME)
                .long(all_output::LONG)
                .about(all_output::HELP)
                .takes_value(true)
                .value_name(all_output::VALUE_NAME)
        )
        .arg(
            Arg::new(verbose::NAME)
                .short(verbose::SHORT)
                .about(verbose::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::new(debug::NAME)
                .short(debug::SHORT)
                .about(debug::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::new(reason::NAME)
                .long(reason::LONG)
                .about(reason::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::new(open::NAME)
                .long(open::LONG)
                .about(open::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::new(packet_trace::NAME)
                .long(packet_trace::LONG)
                .about(packet_trace::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::new(iflist::NAME)
                .long(iflist::LONG)
                .about(iflist::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::new(append_output::NAME)
                .long(append_output::LONG)
                .about(append_output::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::new(resume::NAME)
                .long(resume::LONG)
                .about(resume::HELP)
                .takes_value(true)
                .value_name(resume::VALUE_NAME)
        )
        .arg(
            Arg::new(stylesheet::NAME)
                .long(stylesheet::LONG)
                .about(stylesheet::HELP)
                .takes_value(true)
                .value_name(stylesheet::VALUE_NAME)
        )
        .arg(
            Arg::new(webxml::NAME)
                .long(webxml::LONG)
                .about(webxml::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::new(no_stylesheet::NAME)
                .long(no_stylesheet::LONG)
                .about(no_stylesheet::HELP)
                .takes_value(false)
        )
        .group(
            ArgGroup::new(output::NAME)
                .args(&[
                    normal_output::NAME, XML_output::NAME, script_kiddie_output::NAME,
                    grepable_output::NAME, all_output::NAME, verbose::NAME,
                    debug::NAME, reason::NAME, open::NAME, packet_trace::NAME,
                    iflist::NAME, append_output::NAME, resume::NAME,
                    stylesheet::NAME, webxml::NAME, no_stylesheet::NAME
                ])
                .multiple(true)
        )
}