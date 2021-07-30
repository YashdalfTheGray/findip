use std::net::{IpAddr, Ipv4Addr};

use crate::ip_error::IpError;

pub struct IpQueryParams {
    pub services: Vec<String>,
}

pub fn run_ip_query(params: IpQueryParams) -> Result<IpAddr, IpError> {
    let _ips: Vec<String> = params
        .services
        .iter()
        .map(|service| match reqwest::blocking::get(service) {
            Ok(response) => response.text().unwrap_or("".to_string()),
            Err(_) => "".to_string(),
        })
        .filter(|maybe_ip| !maybe_ip.is_empty())
        .collect();

    Ok(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
}
