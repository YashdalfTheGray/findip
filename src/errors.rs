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

#[derive(Debug, Clone)]
pub struct IpConflict {
    ips: Vec<String>,
}

impl fmt::Display for IpConflict {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Multiple IPs were reported back from the list of services\n{:#?}",
            self.ips_as_string_list()
        )
    }
}

impl Error for IpConflict {}

impl IpConflict {
    pub fn new(ips: Vec<String>) -> Self {
        IpConflict { ips }
    }

    fn ips_as_string_list(&self) -> String {
        self.ips.join("\n")
    }
}
