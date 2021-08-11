use clap::App;
use crate::constants::AFTER_HELP;

pub fn add_footer(app: App<'static>) -> App<'static> {
    app
        .after_help(AFTER_HELP)
}
