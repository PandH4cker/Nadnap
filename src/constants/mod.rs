pub const USAGE_STR: &str = concat!(clap::crate_name!(), " [Scan Type(s)] [Options] target specification");

pub mod args {
    pub mod input_list {
        pub const NAME: &str = "Input List";
        pub const LONG: &str = "iL";
        pub const VALUE_NAME: &str = "filename";
        pub const HELP: &str = "Input from list of hosts/networks";
        pub const NUMBER_OF_VALUES: u64 = 1_u64;
    }

    pub mod input_random {
        pub const NAME: &str = "Input Random";
        pub const LONG: &str = "iR";
        pub const VALUE_NAME: &str = "num hosts";
        pub const HELP: &str = "Choose random targets";
        pub const NUMBER_OF_VALUES: u64 = 1_u64;
    }

    pub mod exclude_hosts {
        pub const NAME: &str = "Exclude Hosts";
        pub const LONG: &str = "exclude";
        pub const VALUE_NAME: &str = "host1[,host2][,host3], ...";
        pub const HELP: &str = "Exclude hosts/networks";
    }

    pub mod exclude_file {
        pub const NAME: &str = "Exclude File";
        pub const LONG: &str = "exludefile";
        pub const VALUE_NAME: &str = "exclude_file";
        pub const HELP: &str = "Exclude list from file";
        pub const NUMBER_OF_VALUES: u64 = 1_64;
    }

    pub mod list_scan {
        pub const NAME: &str = "List Scan";
        pub const LONG: &str = "sL";
        pub const HELP: &str = "List Scan - simply list targets to scan";
    }
    pub mod ping_scan {
        pub const NAME: &str = "Ping Scan";
        pub const LONG: &str = "sn";
        pub const HELP: &str = "Ping Scan - disable port scan";
    }
    pub mod skip_host_discovery {
        pub const NAME: &str = "Skip Host Discovery";
        pub const LONG: &str = "Pn";
        pub const HELP: &str = "Treat all hosts as online -- skip host discovery";
    }
}

pub mod groups {
    pub mod target_specification {
        pub const NAME: &str = "Target Specification";
    }

    pub mod host_discovery {
        pub const NAME: &str = "Host Discovery";
    }
}