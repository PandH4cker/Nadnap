use clap::{
    App, Arg, ArgGroup
};
use crate::constants::{args::*, groups::*};

pub fn add_misc(app: App<'static>) -> App<'static> {
    app
        .help_heading(misc::NAME)
        .arg(
            Arg::new(ipv6_enable::NAME)
                .short(ipv6_enable::SHORT)
                .about(ipv6_enable::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::new(os_ver_script_traceroute_enable::NAME)
                .short(os_ver_script_traceroute_enable::SHORT)
                .about(os_ver_script_traceroute_enable::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::new(datadir::NAME)
                .long(datadir::LONG)
                .about(datadir::HELP)
                .takes_value(true)
                .value_name(datadir::VALUE_NAME)
        )
        .arg(
            Arg::new(send_eth::NAME)
                .long(send_eth::LONG)
                .about(send_eth::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::new(send_ip::NAME)
                .long(send_ip::LONG)
                .about(send_ip::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::new(privileged::NAME)
                .long(privileged::LONG)
                .about(privileged::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::new(unprivileged::NAME)
                .long(unprivileged::LONG)
                .about(unprivileged::HELP)
                .takes_value(false)
        )
        .group(
            ArgGroup::new(misc::NAME)
                .args(&[
                    ipv6_enable::NAME, os_ver_script_traceroute_enable::NAME,
                    datadir::NAME, send_eth::NAME, send_ip::NAME, privileged::NAME,
                    unprivileged::NAME
                ])
                .multiple(true)
        )
}