use std::net::{IpAddr, Ipv4Addr};

pub mod properties;

pub fn hello_world(maybe_name: Option<String>) {
    match maybe_name {
        Some(name) => println!("Hello, {}!", name),
        None => println!("Hello, World!"),
    }
}

pub trait IPNotifier {
    fn notify();
    fn notify_on_change();
}

pub fn find_external_ip(_config: properties::ConfigFile) -> std::net::IpAddr {
    return IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
}

pub fn schedule_ip_notification(config: properties::ConfigFile) {
    println!("{:?}", config);
    println!("{:?}", find_external_ip(config))
}
