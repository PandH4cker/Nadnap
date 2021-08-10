pub mod args;
pub mod groups;
pub mod regex;

pub const USAGE_STR: &str = concat!(clap::crate_name!(), " [Scan Type(s)] [Options] -- {target specification}");
pub const AFTER_HELP: &str = "\
EXAMPLES:\n\
\tnadnap -v -A scanme.nadnap.com\n\
\tnadnap -v -sn 192.168.0.0/16 10.0.0.0/8\n\
\tnadnap -v -iR 10000 -Pn -p 80\n\
SEE THE MAN PAGE (https://nadnap.com/doc) FOR MORE OPTIONS AND EXAMPLES\
";