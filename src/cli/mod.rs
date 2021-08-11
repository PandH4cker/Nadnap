mod base_app;
mod options;

use clap::App;
use base_app::base_app;
use options::{
    footer::add_footer,
    host_discovery::add_host_discovery,
    port_spec_scan_order::add_port_spec_scan_order,
    scan_techniques::add_scan_techniques,
    script_scan::add_script_scan,
    service_ver_detection::add_service_ver_detection,
    target_specification::add_target_specification
};

pub fn generate_app() -> App<'static> {
    add_footer(
        add_script_scan(
            add_service_ver_detection(
                add_port_spec_scan_order(
                    add_scan_techniques(
                        add_host_discovery(
                            add_target_specification(
                                base_app()
                            )
                        )
                    )
                )
            )
        )
    )
}