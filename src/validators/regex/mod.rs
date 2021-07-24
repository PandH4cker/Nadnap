use lazy_static::lazy_static;
use regex::Regex;

pub fn is_hosts(v: String) -> Result<(), String> {
    lazy_static! {
        static ref HOSTNAME_REGEX: Regex = Regex::new(r"(?x)
            ^([a-zA-Z0-9]|[a-zA-Z0-9][a-zA-Z0-9\-]{0,61}[a-zA-Z0-9])
            (\.([a-zA-Z0-9]|[a-zA-Z0-9][a-zA-Z0-9\-]{0,61}[a-zA-Z0-9]))*$
        ").unwrap();
        static ref IP_REGEX: Regex = Regex::new(r"(?x)
            ^((\d|[1-9]\d|1\d{2}|2[0-4]\d|25[0-5])\.){3}
            (\d|[1-9]\d|1\d{2}|2[0-4]\d|25[0-5])$
        ").unwrap();
        static ref NETINT_REGEX: Regex = Regex::new(r"(?x)
            ^((\d|[1-9]\d|1\d{2}|2[0-4]\d|25[0-5])\.){3}
            (\d|[1-9]\d|1\d{2}|2[0-4]\d|25[0-5])/([1-9]|[12]\d|3[012])$
        ").unwrap();
    }
    match HOSTNAME_REGEX.is_match(&v) || IP_REGEX.is_match(&v) || NETINT_REGEX.is_match(&v) {
        true => {
            Ok(())
        }
        false => {
            Err(format!("{} isn't a host nor an ip nor a network interface", v))
        }
    }
}