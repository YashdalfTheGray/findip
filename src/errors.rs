use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub enum ErrorReason {
    IpConflict(Vec<String>),
    InvalidInput(String),
    NoIpAddressesFound,
    FileWriteFailed(String),
    FileOpenFailed(String),
    S3WriteFailed(String),
    RestRequestFailed(String),
    Generic(String),
}

#[derive(Debug, Clone)]
pub struct IpError {
    reason: ErrorReason,
}

impl fmt::Display for IpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.reason {
            ErrorReason::IpConflict(ips) => {
                write!(
                    f,
                    "Multiple IPs were reported back from the list of services\n{:#?}",
                    ips.join("\n"),
                )
            }
            ErrorReason::InvalidInput(context) => {
                write!(f, "An input was unexpected. Context: {}", context)
            }
            ErrorReason::NoIpAddressesFound => write!(f, "No IP addresses were found in the result storage. Most likely, a query has not been run."),
            ErrorReason::FileWriteFailed(file) => write!(f, "Failed to write IP address to file at path {}", file),
            ErrorReason::FileOpenFailed(file) => write!(f, "Failed to open file at path {}", file),
            ErrorReason::S3WriteFailed(reason) => write!(f, "Failed to write IP address to S3. Reason: {}", reason),
            ErrorReason::RestRequestFailed(reason) => write!(f, "Failed to make a REST request. Reason: {}", reason),
            ErrorReason::Generic(context) => write!(f, "An error was encountered. Context: {}", context),
        }
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
