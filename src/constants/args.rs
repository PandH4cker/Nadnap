pub mod target {
    pub const NAME: &str = "Target";
    pub const VALUE_DELIMITER: &str = " ";
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
pub mod service_version_info {
    pub const NAME: &str = "Service/Version info";
    pub const LONG: &str = "sV";
    pub const HELP: &str = "Probe open ports to determine service/version info";
}
pub mod version_intensity {
    pub const NAME: &str = "Version intensity";
    pub const LONG: &str = "version-intensity";
    pub const HELP: &str = "Set from 0 (light) to 9 (try all probes)";
    pub const VALUE_NAME: &str = "level";
    pub const POSSIBLE_VALUES: &[&str] = &[
        "1", "2", "3", "4", "5",
        "6", "7", "8", "9"
    ];
}
pub mod version_light {
    pub const NAME: &str = "Version light";
    pub const LONG: &str = "version-light";
    pub const HELP: &str = "Limit to most likely probes (intensity 2)";
}
pub mod version_all {
    pub const NAME: &str = "Version all";
    pub const LONG: &str = "version-all";
    pub const HELP: &str = "Try every single probe (intensity 9)";
}
pub mod version_trace {
    pub const NAME: &str = "Version trace";
    pub const LONG: &str = "version-trace";
    pub const HELP: &str = "Show detailed version scan activity (for debugging)";
}
pub mod default_script {
    pub const NAME: &str = "Default script";
    pub const LONG: &str = "sC";
    pub const HELP: &str = "equivalent to --script=default";
}
pub mod script {
    pub const NAME: &str = "Script";
    pub const LONG: &str = "script";
    pub const HELP: &str = "<scripts> is a comma separated list of \
    directories, script-files or script-categories";
    pub const VALUE_NAME: &str = "scripts";
    pub const VALUE_DELIMITER: &str = ",";
}
pub mod script_args {
    pub const NAME: &str = "Script args";
    pub const LONG: &str = "script-args";
    pub const HELP: &str = "provide arguments to scripts";
    pub const VALUE_NAME: &str = "n1=v1,[n2=v2,...]";
    pub const VALUE_DELIMITER: &str = ",";
}
pub mod script_args_file {
    pub const NAME: &str = "Script args file";
    pub const LONG: &str = "script-args-file";
    pub const HELP: &str = "provide Nadnap script args in a file";
    pub const VALUE_NAME: &str = "filename";
}
pub mod script_trace {
    pub const NAME: &str = "Script trace";
    pub const LONG: &str = "script-trace";
    pub const HELP: &str = "Show all data sent and received";
}
pub mod script_update_db {
    pub const NAME: &str = "Script update db";
    pub const LONG: &str = "script-updatedb";
    pub const HELP: &str = "Update the script database.";
}
pub mod script_help {
    pub const NAME: &str = "Script help";
    pub const LONG: &str = "script-help";
    pub const HELP: &str = "Show help about scripts.\n\
    <scripts> is a comma-separated list of script-files or script-categories.";
    pub const VALUE_NAME: &str = "scripts";
    pub const VALUE_DELIMITER: &str = ",";
}
pub mod enable_os_detection {
    pub const NAME: &str = "Enable OS Detection";
    pub const SHORT: char = 'O';
    pub const HELP: &str = "Enable OS detection";
}
pub mod os_scan_limit {
    pub const NAME: &str = "OS Scan Limit";
    pub const LONG: &str = "osscan-limit";
    pub const HELP: &str = "Limit OS detection to promising targets";
}
pub mod os_scan_guess {
    pub const NAME: &str = "OS Scan Guess";
    pub const LONG: &str = "osscan-guess";
    pub const HELP: &str = "Guess OS more aggressively";
}
pub mod timing_template {
    pub const NAME: &str = "Timing Template";
    pub const SHORT: char = 'T';
    pub const VALUE_NAME: &str = "0-5";
    pub const HELP: &str = "Set timing template (higher is faster)";
    pub const POSSIBLE_VALUES: &[&str] = &["0", "1", "2", "3", "4", "5"];
}
pub mod min_hostgroup {
    pub const NAME: &str = "Minimum Hostgroup";
    pub const LONG: &str = "min-hostgroup";
    pub const VALUE_NAME: &str = "size";
    pub const HELP: &str = "Parallel host scan group sizes (Min)";
}
pub mod max_hostgroup {
    pub const NAME: &str = "Maximum Hostgroup";
    pub const LONG: &str = "max-hostgroup";
    pub const VALUE_NAME: &str = "size";
    pub const HELP: &str = "Parallel host scan group sizes (Max)";
}
pub mod min_parallelism {
    pub const NAME: &str = "Minimum Parallelism";
    pub const LONG: &str = "min-parallelism";
    pub const VALUE_NAME: &str = "numprobes";
    pub const HELP: &str = "Probe parallelization (Min)";
}
pub mod max_parallelism {
    pub const NAME: &str = "Maximum Parallelism";
    pub const LONG: &str = "max-parallelism";
    pub const VALUE_NAME: &str = "numprobes";
    pub const HELP: &str = "Probe parallelization (Max)";
}
pub mod min_rtt_timeout {
    pub const NAME: &str = "Minimum Round Trip Time";
    pub const LONG: &str = "min-rtt-timeout";
    pub const VALUE_NAME: &str = "time";
    pub const HELP: &str = "Specifies probe round trip time. (Min)";
}
pub mod max_rtt_timeout {
    pub const NAME: &str = "Maximum Round Trip Time";
    pub const LONG: &str = "max-rtt-timeout";
    pub const VALUE_NAME: &str = "time";
    pub const HELP: &str = "Specifies probe round trip time. (Max)";
}
pub mod initial_rtt_timeout {
    pub const NAME: &str = "Initial Round Trip Time";
    pub const LONG: &str = "initial-rtt-timeout";
    pub const VALUE_NAME: &str = "time";
    pub const HELP: &str = "Specifies probe round trip time. (Initial)";
}
pub mod max_retries {
    pub const NAME: &str = "Maximum Retries";
    pub const LONG: &str = "max-retries";
    pub const VALUE_NAME: &str = "tries";
    pub const HELP: &str = "Caps number of port scan probe retransmissions.";
}
pub mod host_timeout {
    pub const NAME: &str = "Host Timeout";
    pub const LONG: &str = "host-timeout";
    pub const VALUE_NAME: &str = "time";
    pub const HELP: &str = "Give up on target after this long";
}
pub mod scan_delay {
    pub const NAME: &str = "Scan Delay";
    pub const LONG: &str = "scan-delay";
    pub const VALUE_NAME: &str = "time";
    pub const HELP: &str = "Adjust delay between probes";
}
pub mod max_scan_delay {
    pub const NAME: &str = "Maximum Scan Delay";
    pub const LONG: &str = "max-scan-delay";
    pub const VALUE_NAME: &str = "time";
    pub const HELP: &str = "Adjust delay between probes (Max)";
}
pub mod min_rate {
    pub const NAME: &str = "Minimum Rate";
    pub const LONG: &str = "min-rate";
    pub const VALUE_NAME: &str = "number";
    pub const HELP: &str = "Send packets no slower than <number> per second";
}
pub mod max_rate {
    pub const NAME: &str = "Maximum Rate";
    pub const LONG: &str = "max-rate";
    pub const VALUE_NAME: &str = "number";
    pub const HELP: &str = "Send packets no faster than <number> per second";
}
pub mod fragment_packets {
    pub const NAME: &str = "Fragment Packets";
    pub const SHORT: char = 'f';
    pub const HELP: &str = "Fragment packets";
}
pub mod fragment_packets_mtu {
    pub const NAME: &str = "Fragment Packets MTU";
    pub const LONG: &str = "mtu";
    pub const VALUE_NAME: &str = "val";
    pub const HELP: &str = "Fragment packets with given MTU";
}
pub mod decoy {
    pub const NAME: &str = "Decoy";
    pub const SHORT: char = 'D';
    pub const HELP: &str = "Cloak a scan with decoys";
    pub const VALUE_NAME: &str = "decoy1,decoy2[,ME],...";
    pub const VALUE_DELIMITER: &str = ",";
}
pub mod ip_spoof {
    pub const NAME: &str = "IP Spoof";
    pub const SHORT: char = 'S';
    pub const HELP: &str = "Spoof source address";
    pub const VALUE_NAME: &str = "IP_Address";
}
pub mod use_interface {
    pub const NAME: &str = "Use Interface";
    pub const SHORT: char = 'e';
    pub const HELP: &str = "Use specified interface";
    pub const VALUE_NAME: &str = "iface";
}
pub mod use_port {
    pub const NAME: &str = "Use Port";
    pub const SHORT: char = 'g';
    pub const LONG: &str = "source-port";
    pub const VALUE_NAME: &str = "portnum";
    pub const HELP: &str = "Use given port number";
}
pub mod proxies {
    pub const NAME: &str = "Proxies";
    pub const LONG: &str = "proxies";
    pub const VALUE_NAME: &str = "url1,[url2],...";
    pub const VALUE_DELIMITER: &str = ",";
    pub const HELP: &str = "Relay connections through HTTP/SOCKS4 proxies";
}
pub mod data {
    pub const NAME: &str = "Data";
    pub const LONG: &str = "data";
    pub const VALUE_NAME: &str = "hex string";
    pub const HELP: &str = "Append a custom payload to sent packets";
}
pub mod data_string {
    pub const NAME: &str = "Data String";
    pub const LONG: &str = "data-string";
    pub const VALUE_NAME: &str = "string";
    pub const HELP: &str = "Append a custom ASCII string to sent packets";
}
pub mod data_length {
    pub const NAME: &str = "Data Length";
    pub const LONG: &str = "data-length";
    pub const VALUE_NAME: &str = "num";
    pub const HELP: &str = "Append random data to sent packets";
}
pub mod ip_options {
    pub const NAME: &str = "IP Options";
    pub const LONG: &str = "ip-options";
    pub const VALUE_NAME: &str = "options";
    pub const HELP: &str = "Send packets with specified ip options";
}
pub mod ttl {
    pub const NAME: &str = "ttl";
    pub const LONG: &str = "ttl";
    pub const VALUE_NAME: &str = "val";
    pub const HELP: &str = "Set IP time-to-live field";
}
pub mod spoof_mac {
    pub const NAME: &str = "Spoof MAC";
    pub const LONG: &str = "spoof-mac";
    pub const VALUE_NAME: &str = "mac address/prefix/vendor name";
    pub const HELP: &str = "Spoof your MAC address";
}
pub mod badsum {
    pub const NAME: &str = "Badsum";
    pub const LONG: &str = "badsum";
    pub const HELP: &str = "Send packets with a bogus TCP/UDP/SCTP checksum";
}
pub mod normal_output {
    pub const NAME: &str = "Normal Output";
    pub const LONG: &str = "oN";
    pub const VALUE_NAME: &str = "file";
    pub const HELP: &str = "Output scan in normal format to the given filename.";
}
pub mod XML_output {
    pub const NAME: &str = "XML Output";
    pub const LONG: &str = "oX";
    pub const VALUE_NAME: &str = "file";
    pub const HELP: &str = "Output scan in XML format to the given filename.";
}
pub mod script_kiddie_output {
    pub const NAME: &str = "Script Kiddie Output";
    pub const LONG: &str = "oS";
    pub const VALUE_NAME: &str = "file";
    pub const HELP: &str = "Output scan in s|<rIpt kIddi3 format to the given filename.";
}
pub mod grepable_output {
    pub const NAME: &str = "Grepable Output";
    pub const LONG: &str = "oG";
    pub const VALUE_NAME: &str = "file";
    pub const HELP: &str = "Output scan in grepable format to the given filename.";
}
pub mod all_output {
    pub const NAME: &str = "All Output";
    pub const LONG: &str = "oA";
    pub const VALUE_NAME: &str = "basename";
    pub const HELP: &str = "Output in the three major formats at once";
}
pub mod verbose {
    pub const NAME: &str = "Verbose";
    pub const SHORT: char = 'v';
    pub const HELP: &str = "Increase verbosity level (use -vv or more for greater effect)";
}
pub mod debug {
    pub const NAME: &str = "Debug";
    pub const SHORT: char = 'd';
    pub const HELP: &str = "Increase debugging level (use -dd or more for greater effect)";
}
pub mod reason {
    pub const NAME: &str = "Reason";
    pub const LONG: &str = "reason";
    pub const HELP: &str = "Display the reason a port is in a particular state";
}
pub mod open {
    pub const NAME: &str = "Open";
    pub const LONG: &str = "open";
    pub const HELP: &str = "Only show open (or possibly open) ports";
}
pub mod packet_trace {
    pub const NAME: &str = "Packet Trace";
    pub const LONG: &str = "packet-trace";
    pub const HELP: &str = "Show all packets sent and received";
}
pub mod iflist {
    pub const NAME: &str = "Iflist";
    pub const LONG: &str = "iflist";
    pub const HELP: &str = "Print host interfaces and routes (for debugging)";
}
pub mod append_output {
    pub const NAME: &str = "Append Output";
    pub const LONG: &str = "append-output";
    pub const HELP: &str = "Append to rather than clobber specified output files";
}
pub mod resume {
    pub const NAME: &str = "Resume";
    pub const LONG: &str = "resume";
    pub const VALUE_NAME: &str = "filename";
    pub const HELP: &str = "Resume an aborted scan";
}
pub mod stylesheet {
    pub const NAME: &str = "Stylesheet";
    pub const LONG: &str = "stylesheet";
    pub const VALUE_NAME: &str = "path/URL";
    pub const HELP: &str = "XSL stylesheet to transform XML output to HTML";
}
pub mod webxml {
    pub const NAME: &str = "WebXML";
    pub const LONG: &str = "webxml";
    pub const HELP: &str = "Reference stylesheet from Nmap.Org for more portable XML";
}
pub mod no_stylesheet {
    pub const NAME: &str = "No Stylesheet";
    pub const LONG: &str = "no-stylesheet";
    pub const HELP: &str = "Prevent associating of XSL stylesheet w/XML output";
}
pub mod ipv6_enable {
    pub const NAME: &str = "IPv6 Enable";
    pub const SHORT: char = '6';
    pub const HELP: &str = "Enable IPv6 scanning";
}
pub mod os_ver_script_traceroute_enable {
    pub const NAME: &str = "OS, Version, Script and Traceroute Enable";
    pub const SHORT: char = 'A';
    pub const HELP: &str = "Enable OS detection, version detection, script scanning, and traceroute";
}
pub mod datadir {
    pub const NAME: &str = "Data Directory";
    pub const LONG: &str = "datadir";
    pub const VALUE_NAME: &str = "dirname";
    pub const HELP: &str = "Specify custom Nmap data file location";
}
pub mod send_eth {
    pub const NAME: &str = "Send ETH";
    pub const LONG: &str = "send-eth";
    pub const HELP: &str = "Send using raw ethernet frames";
}
pub mod send_ip {
    pub const NAME: &str = "Send IP";
    pub const LONG: &str = "send-ip";
    pub const HELP: &str = "Send using IP packets";
}
pub mod privileged {
    pub const NAME: &str = "Privileged";
    pub const LONG: &str = "privileged";
    pub const HELP: &str = "Assume that the user is fully privileged";
}
pub mod unprivileged {
    pub const NAME: &str = "Unprivileged";
    pub const LONG: &str = "unprivileged";
    pub const HELP: &str = "Assume the user lacks raw socket privileges";
}