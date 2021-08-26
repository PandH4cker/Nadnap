use clap::{
    App, Arg, ArgGroup
};
use crate::constants::{args::*, groups::*};

pub fn add_os_detection(app: App<'static>) -> App<'static> {
    app
        .help_heading(os_detection::NAME)
        .arg(
            Arg::new(enable_os_detection::NAME)
                .short(enable_os_detection::SHORT)
                .about(enable_os_detection::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::new(os_scan_limit::NAME)
                .long(os_scan_limit::LONG)
                .about(os_scan_limit::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::new(os_scan_guess::NAME)
                .long(os_scan_guess::LONG)
                .about(os_scan_guess::HELP)
                .takes_value(false)
        )
        .group(
            ArgGroup::new(os_detection::NAME)
                .args(&[enable_os_detection::NAME, os_scan_limit::NAME, os_scan_guess::NAME])
                .multiple(true)
        )
}