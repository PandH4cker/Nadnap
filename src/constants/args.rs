pub mod target {
    pub const NAME: &str = "Target";
}
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
    pub const VALUE_NAME: &str = "host1[,host2][,host3],...";
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
pub mod tcp_syn_discovery {
    pub const NAME: &str = "TCP SYN Discovery";
    pub const LONG: &str = "PS";
    pub const HELP: &str = "TCP SYN discovery to given ports";
    pub const VALUE_NAME: &str = "portlist";
}
pub mod tcp_ack_discovery {
    pub const NAME: &str = "TCP ACK Discovery";
    pub const LONG: &str = "PA";
    pub const HELP: &str = "TCP ACK discovery to given ports";
    pub const VALUE_NAME: &str = "portlist";
}
pub mod udp_discovery {
    pub const NAME: &str = "UDP Discovery";
    pub const LONG: &str = "PU";
    pub const HELP: &str = "UDP discovery to given ports";
    pub const VALUE_NAME: &str = "portlist";
}
pub mod sctp_discovery {
    pub const NAME: &str = "SCTP Discovery";
    pub const LONG: &str = "PY";
    pub const HELP: &str = "SCTP discovery to given ports";
    pub const VALUE_NAME: &str = "portlist";
}
pub mod icmp_echo_discovery {
    pub const NAME: &str = "ICMP Echo Discovery";
    pub const LONG: &str = "PE";
    pub const HELP: &str = "ICMP echo request discovery probes";
}
pub mod icmp_timestamp_discovery {
    pub const NAME: &str = "ICMP Timestamp Discovery";
    pub const LONG: &str = "PP";
    pub const HELP: &str = "ICMP Timestamp request discovery probes";
}
pub mod icmp_netmask_discovery {
    pub const NAME: &str = "ICMP Netmask Discovery";
    pub const LONG: &str = "PM";
    pub const HELP: &str = "ICMP Netmask request discovery probes";
}
pub mod ip_protocol_ping {
    pub const NAME: &str = "IP Protocol Ping";
    pub const LONG: &str = "PO";
    pub const HELP: &str = "IP Protocol Ping";
    pub const VALUE_NAME: &str = "protocol list";
}
pub mod never_resolve {
    pub const NAME: &str = "Never Resolve";
    pub const SHORT: char = 'n';
    pub const HELP: &str = "Never do DNS resolution";
}
pub mod always_resolve {
    pub const NAME: &str = "Always Resolve";
    pub const SHORT: char = 'R';
    pub const HELP: &str = "Always do DNS resolution";
}
pub mod dns_servers {
    pub const NAME: &str = "DNS Servers";
    pub const LONG: &str = "dns-servers";
    pub const HELP: &str = "Specify custom DNS servers";
    pub const VALUE_NAME: &str = "serv1[,serv2],...";
}
pub mod system_dns {
    pub const NAME: &str = "System DNS";
    pub const LONG: &str = "system-dns";
    pub const HELP: &str = "Use OS's DNS resolver";
}
pub mod traceroute {
    pub const NAME: &str = "Traceroute";
    pub const LONG: &str = "traceroute";
    pub const HELP: &str = "Trace hop path to each host";
}
pub mod tcp_syn_scan {
    pub const NAME: &str = "TCP SYN Scan";
    pub const LONG: &str = "sS";
    pub const HELP: &str = "TCP SYN scan";
}
pub mod tcp_connect_scan {
    pub const NAME: &str = "TCP Connect() Scan";
    pub const LONG: &str = "sT";
    pub const HELP: &str = "TCP Connect() scan";
}
pub mod tcp_ack_scan {
    pub const NAME: &str = "TCP ACK Scan";
    pub const LONG: &str = "sA";
    pub const HELP: &str = "TCP ACK scan";
}
pub mod tcp_window_scan {
    pub const NAME: &str = "TCP Window Scan";
    pub const LONG: &str = "sW";
    pub const HELP: &str = "TCP Window scan";
}
pub mod tcp_maimon_scan {
    pub const NAME: &str = "TCP Maimon Scan";
    pub const LONG: &str = "sM";
    pub const HELP: &str = "TCP Maimon scan";
}
pub mod udp_scan {
    pub const NAME: &str = "UDP Scan";
    pub const LONG: &str = "sU";
    pub const HELP: &str = "UDP scan";
}
pub mod tcp_null_scan {
    pub const NAME: &str = "TCP Null Scan";
    pub const LONG: &str = "sN";
    pub const HELP: &str = "TCP Null scan";
}
pub mod tcp_fin_scan {
    pub const NAME: &str = "TCP FIN Scan";
    pub const LONG: &str = "sF";
    pub const HELP: &str = "TCP FIN scan";
}
pub mod tcp_xmas_scan {
    pub const NAME: &str = "TCP Xmas Scan";
    pub const LONG: &str = "sX";
    pub const HELP: &str = "TCP Xmas scan";
}
pub mod scan_flags {
    pub const NAME: &str = "TCP scan flags";
    pub const LONG: &str = "scanflags";
    pub const HELP: &str = "Customize TCP scan flags";
    pub const VALUE_NAME: &str = "flags";
}
pub mod idle_scan {
    pub const NAME: &str = "Idle scan";
    pub const LONG: &str = "sI";
    pub const HELP: &str = "Idle scan";
    pub const VALUE_NAME: &str = "zombie host[:probeport]";
}
pub mod sctp_init_scan {
    pub const NAME: &str = "SCTP INIT scan";
    pub const LONG: &str = "sY";
    pub const HELP: &str = "SCTP INIT scan";
}
pub mod sctp_cookie_echo_scan {
    pub const NAME: &str = "SCTP COOKIE-ECHO scan";
    pub const LONG: &str = "sZ";
    pub const HELP: &str = "SCTP COOKIE-ECHO scan";
}
pub mod ip_protocol_scan {
    pub const NAME: &str = "IP Protocol scan";
    pub const LONG: &str = "sO";
    pub const HELP: &str = "IP protocol scan";
}
pub mod ftp_bounce_scan {
    pub const NAME: &str = "FTP Bounce scan";
    pub const SHORT: char = 'b';
    pub const HELP: &str = "FTP bounce scan";
    pub const VALUE_NAME: &str = "FTP relay host";
}
pub mod port_ranges {
    pub const NAME: &str = "Port ranges";
    pub const SHORT: char = 'p';
    pub const HELP: &str = "Only scan specified ports";
    pub const VALUE_NAME: &str = "port ranges";
}
pub mod exclude_ports {
    pub const NAME: &str = "Exclude ports";
    pub const LONG: &str = "exclude-ports";
    pub const HELP: &str = "Exclude the specified ports from scanning";
    pub const VALUE_NAME: &str = "port ranges";
}
pub mod fast_mode {
    pub const NAME: &str = "Fast mode";
    pub const SHORT: char = 'F';
    pub const HELP: &str = "Fast mode - Scan fewer ports than the default scan";
}
pub mod dont_randomize {
    pub const NAME: &str = "Don't randomize";
    pub const SHORT: char = 'r';
    pub const HELP: &str = "Scan ports consecutively - don't randomize";
}
pub mod top_ports {
    pub const NAME: &str = "Top ports";
    pub const LONG: &str = "top-ports";
    pub const HELP: &str = "Scan <number> most common ports";
    pub const VALUE_NAME: &str = "number";
}
pub mod port_ratio {
    pub const NAME: &str = "Port ratio";
    pub const LONG: &str = "port-ratio";
    pub const HELP: &str = "Scan ports more common than <ratio>";
    pub const VALUE_NAME: &str = "ratio";
}