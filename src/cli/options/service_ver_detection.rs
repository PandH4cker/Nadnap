use clap::{
    App, Arg, ArgGroup
};
use crate::constants::{args::*, groups::*};

pub fn add_service_ver_detection(app: App<'static>) -> App<'static> {
    app
        .help_heading(service_ver_detection::NAME)
        .arg(
            Arg::new(service_version_info::NAME)
                .about(service_version_info::HELP)
                .takes_value(false)
                .long(service_version_info::LONG)
        )
        .arg(
            Arg::new(version_intensity::NAME)
                .about(version_intensity::HELP)
                .long(version_intensity::LONG)
                .takes_value(true)
                .value_name(version_intensity::VALUE_NAME)
                .possible_values(version_intensity::POSSIBLE_VALUES)
                .conflicts_with_all(&[version_all::NAME, version_light::NAME])
        )
        .arg(
            Arg::new(version_light::NAME)
                .about(version_light::HELP)
                .long(version_light::LONG)
                .takes_value(false)
                .conflicts_with_all(&[version_intensity::NAME, version_all::NAME])
        )
        .arg(
            Arg::new(version_all::NAME)
                .about(version_all::HELP)
                .long(version_all::LONG)
                .takes_value(false)
                .conflicts_with_all(&[version_light::NAME, version_intensity::NAME])
        )
        .arg(
            Arg::new(version_trace::NAME)
                .about(version_trace::HELP)
                .long(version_trace::LONG)
                .takes_value(false)
        )
        .group(
            ArgGroup::new(service_ver_detection::NAME)
                .args(&[
                    service_version_info::NAME, version_intensity::NAME, version_light::NAME,
                    version_all::NAME, version_trace::NAME
                ])
                .multiple(true)
        )
}