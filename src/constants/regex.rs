pub const HOSTNAME_REGEX_STR : &str = r"(?x)
            ^([a-zA-Z0-9]|[a-zA-Z0-9][a-zA-Z0-9\-]{0,61}[a-zA-Z0-9])
            (\.([a-zA-Z0-9]|[a-zA-Z0-9][a-zA-Z0-9\-]{0,61}[a-zA-Z0-9]))*$
";
pub const IP_REGEX_STR : &str = r"(?x)
            ^((\d|[1-9]\d|1\d{2}|2[0-4]\d|25[0-5])\.){3}
            (\d|[1-9]\d|1\d{2}|2[0-4]\d|25[0-5])$
";
pub const NETINT_REGEX_STR : &str = r"(?x)
            ^((\d|[1-9]\d|1\d{2}|2[0-4]\d|25[0-5])\.){3}
            (\d|[1-9]\d|1\d{2}|2[0-4]\d|25[0-5])/([1-9]|[12]\d|3[012])$
";