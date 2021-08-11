use clap::{
    crate_name, crate_version, crate_authors, crate_description,
    App, AppSettings
};
use crate::constants::USAGE_STR;

pub fn base_app() -> App<'static> {
    App::new(crate_name!())
        .override_usage(USAGE_STR)
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::ArgRequiredElseHelp)
}