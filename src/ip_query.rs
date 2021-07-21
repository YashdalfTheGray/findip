use std::net::{IpAddr, Ipv4Addr};

use crate::ip_error::IpError;

pub struct IpQueryParams {
    pub services: Vec<String>,
}

pub fn run_ip_query(_params: IpQueryParams) -> Result<IpAddr, IpError> {
    Ok(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
}
