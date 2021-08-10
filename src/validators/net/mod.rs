use lazy_static::lazy_static;
use regex::RegexSet;
use crate::constants::regex::{
    HOSTNAME_REGEX_STR,
    IP_REGEX_STR,
    NETINT_REGEX_STR
};

pub fn is_hosts(v: &str) -> Result<(), String> {
    lazy_static! {
        static ref HOST_REGEXSET : RegexSet = RegexSet::new(&[
            HOSTNAME_REGEX_STR,
            IP_REGEX_STR,
            NETINT_REGEX_STR
        ]).unwrap();
    }
    match HOST_REGEXSET.is_match(v) {
        true => {
            Ok(())
        }
        false => {
            Err(format!("{} isn't a host nor an ip nor a network interface", v))
        }
    }
}