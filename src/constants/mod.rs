pub mod args;
pub mod groups;
pub mod regex;

pub const USAGE_STR: &str = concat!(clap::crate_name!(), " [Scan Type(s)] [Options] -- {target specification}");