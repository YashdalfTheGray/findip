use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub enum ErrorReason {
    IpConflict(Vec<String>),
    InvalidInput(String),
    NoIpAddressesFound,
    Generic(String),
}

#[derive(Debug, Clone)]
pub struct IpError {
    reason: ErrorReason,
}

impl fmt::Display for IpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IP Address not found.",)
    }
}

impl Error for IpError {}

impl IpError {
    pub fn new(reason: ErrorReason) -> Self {
        IpError { reason }
    }
}

#[derive(Debug, Clone)]
pub struct IpConflictError {
    ips: Vec<String>,
}

impl fmt::Display for IpConflictError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Multiple IPs were reported back from the list of services\n{:#?}",
            self.ips_as_string_list()
        )
    }
}

impl Error for IpConflictError {}

impl IpConflictError {
    pub fn new(ips: Vec<String>) -> Self {
        IpConflictError { ips }
    }

    fn ips_as_string_list(&self) -> String {
        self.ips.join("\n")
    }
}
