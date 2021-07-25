use std::net::{IpAddr, Ipv4Addr};

pub mod config_file;
pub mod ip_error;
pub mod ip_query;
pub mod ip_result;
pub mod notifier;

pub fn find_external_ip(_config: config_file::ConfigFile) -> std::net::IpAddr {
    IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))
}

pub fn schedule_ip_notification(config: config_file::ConfigFile) {
    println!("{:?}", config);
    println!("{:?}", find_external_ip(config))
}
