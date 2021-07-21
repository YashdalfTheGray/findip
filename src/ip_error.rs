use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct IpError {}

impl fmt::Display for IpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IP Address not found.",)
    }
}

impl Error for IpError {}

impl IpError {
    pub fn new() -> Self {
        IpError {}
    }
}
