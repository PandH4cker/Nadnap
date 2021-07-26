use clap::{crate_name, crate_version, crate_authors, crate_description, App, Arg, ArgGroup};
use crate::constants::{
    USAGE_STR,
    args::*,
    groups::*
};
use crate::validators;

pub fn generate_app() -> App<'static, 'static> {
    App::new(crate_name!())
        .usage(USAGE_STR)
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())

        /* *
         * TARGET SPECIFICATION
         */
        .arg(
            Arg::with_name(target::NAME)
                .last(true)
                .takes_value(true)
                .value_delimiter(" ")
                .multiple(true)
                .validator(validators::net::is_hosts)
                .required_unless_one(&[input_list::NAME, input_random::NAME])
        )
        .arg(
            Arg::with_name(input_list::NAME)
                .long(input_list::LONG)
                .value_name(input_list::VALUE_NAME)
                .help(input_list::HELP)
                .takes_value(true)
                .number_of_values(input_list::NUMBER_OF_VALUES)
                .validator(validators::fs::is_file)
                .required_unless_one(&[target::NAME, input_random::NAME])
        )
        .arg(
            Arg::with_name(input_random::NAME)
                .long(input_random::LONG)
                .value_name(input_random::VALUE_NAME)
                .help(input_random::HELP)
                .takes_value(true)
                .number_of_values(input_random::NUMBER_OF_VALUES)
                .validator(validators::num::is_positive)
                .required_unless_one(&[input_list::NAME, target::NAME])
        )
        .arg(
            Arg::with_name(exclude_hosts::NAME)
                .long(exclude_hosts::LONG)
                .value_name(exclude_hosts::VALUE_NAME)
                .help(exclude_hosts::HELP)
                .takes_value(true)
                .validator(validators::net::is_hosts)
                .use_delimiter(true)
        )
        .arg(
            Arg::with_name(exclude_file::NAME)
                .long(exclude_file::LONG)
                .value_name(exclude_file::VALUE_NAME)
                .help(exclude_file::HELP)
                .takes_value(true)
                .validator(validators::fs::is_file)
                .number_of_values(exclude_file::NUMBER_OF_VALUES)
        )
        .group(
            ArgGroup::with_name(target_specification::NAME)
                .args(&[
                    target::NAME, input_list::NAME, input_random::NAME,
                    exclude_hosts::NAME, exclude_file::NAME
                ])
                .multiple(true)
        )

        /* *
         * HOST DISCOVERY
         */
        .arg(
            Arg::with_name(list_scan::NAME)
                .long(list_scan::LONG)
                .help(list_scan::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::with_name(ping_scan::NAME)
                .long(ping_scan::LONG)
                .help(ping_scan::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::with_name(skip_host_discovery::NAME)
                .long(skip_host_discovery::LONG)
                .help(skip_host_discovery::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::with_name(tcp_syn_discovery::NAME)
                .long(tcp_syn_discovery::LONG)
                .help(tcp_syn_discovery::HELP)
                .value_name(tcp_syn_discovery::VALUE_NAME)
                .takes_value(true)
        )
        .arg(
            Arg::with_name(tcp_ack_discovery::NAME)
                .long(tcp_ack_discovery::LONG)
                .help(tcp_ack_discovery::HELP)
                .value_name(tcp_ack_discovery::VALUE_NAME)
                .takes_value(true)
        )
        .arg(
            Arg::with_name(udp_discovery::NAME)
                .long(udp_discovery::LONG)
                .help(udp_discovery::HELP)
                .value_name(udp_discovery::VALUE_NAME)
                .takes_value(true)
        )
        .arg(
            Arg::with_name(sctp_discovery::NAME)
                .long(sctp_discovery::LONG)
                .help(sctp_discovery::HELP)
                .value_name(sctp_discovery::VALUE_NAME)
                .takes_value(true)
        )
        .arg(
            Arg::with_name(icmp_echo_discovery::NAME)
                .long(icmp_echo_discovery::LONG)
                .help(icmp_echo_discovery::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::with_name(icmp_timestamp_discovery::NAME)
                .long(icmp_timestamp_discovery::LONG)
                .help(icmp_timestamp_discovery::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::with_name(icmp_netmask_discovery::NAME)
                .long(icmp_netmask_discovery::LONG)
                .help(icmp_netmask_discovery::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::with_name(ip_protocol_ping::NAME)
                .long(ip_protocol_ping::LONG)
                .help(ip_protocol_ping::HELP)
                .takes_value(true)
                .value_name(ip_protocol_ping::VALUE_NAME)
        )
        .arg(
            Arg::with_name(never_resolve::NAME)
                .short(never_resolve::SHORT)
                .help(never_resolve::HELP)
                .takes_value(false)
                .conflicts_with(always_resolve::NAME)
        )
        .arg(
            Arg::with_name(always_resolve::NAME)
                .short(always_resolve::SHORT)
                .help(always_resolve::HELP)
                .takes_value(false)
                .conflicts_with(never_resolve::NAME)
        )
        .arg(
            Arg::with_name(dns_servers::NAME)
                .long(dns_servers::LONG)
                .help(dns_servers::HELP)
                .takes_value(true)
                .value_name(dns_servers::VALUE_NAME)
        )
        .arg(
            Arg::with_name(system_dns::NAME)
                .long(system_dns::LONG)
                .help(system_dns::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::with_name(traceroute::NAME)
                .long(traceroute::LONG)
                .help(traceroute::HELP)
                .takes_value(false)
        )
        .group(
            ArgGroup::with_name(host_discovery::NAME)
                .multiple(true)
                .args(&[
                    list_scan::NAME, ping_scan::NAME, skip_host_discovery::NAME,
                    tcp_syn_discovery::NAME, tcp_ack_discovery::NAME, udp_discovery::NAME,
                    sctp_discovery::NAME, icmp_echo_discovery::NAME, icmp_timestamp_discovery::NAME,
                    icmp_netmask_discovery::NAME, ip_protocol_ping::NAME, never_resolve::NAME,
                    always_resolve::NAME, dns_servers::NAME, system_dns::NAME, traceroute::NAME
                ])
        )

        /* *
         * SCAN TECHNIQUES
         */
        .arg(
            Arg::with_name(tcp_syn_scan::NAME)
                .long(tcp_syn_scan::LONG)
                .help(tcp_syn_scan::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::with_name(tcp_connect_scan::NAME)
                .long(tcp_connect_scan::LONG)
                .help(tcp_connect_scan::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::with_name(tcp_ack_scan::NAME)
                .long(tcp_ack_scan::LONG)
                .help(tcp_ack_scan::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::with_name(tcp_window_scan::NAME)
                .long(tcp_window_scan::LONG)
                .help(tcp_window_scan::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::with_name(tcp_maimon_scan::NAME)
                .long(tcp_maimon_scan::LONG)
                .help(tcp_maimon_scan::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::with_name(udp_scan::NAME)
                .long(udp_scan::LONG)
                .help(udp_scan::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::with_name(tcp_null_scan::NAME)
                .long(tcp_null_scan::LONG)
                .help(tcp_null_scan::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::with_name(tcp_fin_scan::NAME)
                .long(tcp_fin_scan::LONG)
                .help(tcp_fin_scan::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::with_name(tcp_xmas_scan::NAME)
                .long(tcp_xmas_scan::LONG)
                .help(tcp_xmas_scan::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::with_name(scan_flags::NAME)
                .long(scan_flags::LONG)
                .help(scan_flags::HELP)
                .takes_value(true)
                .value_name(scan_flags::VALUE_NAME)
        )
        .arg(
            Arg::with_name(idle_scan::NAME)
                .long(idle_scan::LONG)
                .help(idle_scan::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::with_name(sctp_init_scan::NAME)
                .long(sctp_init_scan::LONG)
                .help(sctp_init_scan::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::with_name(sctp_cookie_echo_scan::NAME)
                .long(sctp_cookie_echo_scan::LONG)
                .help(sctp_cookie_echo_scan::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::with_name(ip_protocol_scan::NAME)
                .long(ip_protocol_scan::LONG)
                .help(ip_protocol_scan::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::with_name(ftp_bounce_scan::NAME)
                .short(ftp_bounce_scan::SHORT)
                .help(ftp_bounce_scan::HELP)
                .takes_value(false)
        )
        .group(
            ArgGroup::with_name(scan_techniques::NAME)
                .args(&[
                    tcp_syn_scan::NAME, tcp_ack_scan::NAME, tcp_connect_scan::NAME,
                    tcp_window_scan::NAME, tcp_maimon_scan::NAME, udp_scan::NAME,
                    tcp_null_scan::NAME, tcp_fin_scan::NAME, tcp_xmas_scan::NAME,
                    scan_flags::NAME, sctp_init_scan::NAME, sctp_cookie_echo_scan::NAME,
                    ip_protocol_scan::NAME, ftp_bounce_scan::NAME
                ])
                .multiple(true)
        )

        /* *
         * PORT SPECIFICATION AND SCAN ORDER
         */
        .arg(
            Arg::with_name(port_ranges::NAME)
                .short(port_ranges::SHORT)
                .help(port_ranges::HELP)
                .takes_value(true)
                .value_name(port_ranges::VALUE_NAME)
        )
        .arg(
            Arg::with_name(exclude_ports::NAME)
                .long(exclude_ports::LONG)
                .help(exclude_ports::HELP)
                .takes_value(true)
                .value_name(exclude_ports::VALUE_NAME)
        )
        .arg(
            Arg::with_name(fast_mode::NAME)
                .short(fast_mode::SHORT)
                .help(fast_mode::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::with_name(dont_randomize::NAME)
                .short(dont_randomize::SHORT)
                .help(dont_randomize::HELP)
                .takes_value(false)
        )
        .arg(
            Arg::with_name(top_ports::NAME)
                .long(top_ports::LONG)
                .help(top_ports::HELP)
                .takes_value(true)
                .value_name(top_ports::VALUE_NAME)
        )
        .arg(
            Arg::with_name(port_ratio::NAME)
                .long(port_ratio::LONG)
                .help(port_ratio::HELP)
                .takes_value(true)
                .value_name(port_ratio::VALUE_NAME)
        )
        .group(
            ArgGroup::with_name(port_specification_scan_order::NAME)
                .args(&[
                    port_ranges::NAME, exclude_ports::NAME, fast_mode::NAME,
                    dont_randomize::NAME, top_ports::NAME, port_ratio::NAME
                ])
                .multiple(true)
        )
}