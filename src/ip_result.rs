use std::{error::Error, fmt, net::IpAddr, sync::Mutex};

use chrono::{DateTime, Utc};

pub trait IpResultStorage {
    type ErrorType;

    fn add_result(self, ip: IpAddr, checked_at: DateTime<Utc>);
    fn get_latest_ip(self) -> Result<IpAddr, Self::ErrorType>;
    fn has_changed(self) -> bool;
}

struct IPResult {
    pub ip: IpAddr,
    pub checked_at: DateTime<Utc>,
}

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

pub struct IPResults {
    results: Mutex<Vec<IPResult>>,
}

impl IPResults {
    pub fn new() -> IPResults {
        IPResults {
            results: Mutex::new(Vec::new()),
        }
    }
}

impl IpResultStorage for IPResults {
    type ErrorType = IpError;

    fn add_result(self, ip: IpAddr, checked_at: DateTime<Utc>) {
        let mut locked_results = self.results.lock().unwrap();
        if (*locked_results).len() >= 2 {
            (*locked_results).truncate(1);
        }
        (*locked_results).push(IPResult { ip, checked_at })
    }

    fn get_latest_ip(self) -> Result<IpAddr, IpError> {
        let locked_results = self.results.lock().unwrap();

        if (*locked_results).len() == 0 {
            Err(IpError::new())
        } else {
            Ok((*locked_results)[0].ip)
        }
    }

    fn has_changed(self) -> bool {
        let locked_results = self.results.lock().unwrap();
        if (*locked_results).len() == 0 {
            false
        } else if (*locked_results).len() < 2 {
            true
        } else {
            let old_ip = (*locked_results)[1].ip;
            let new_ip = (*locked_results)[0].ip;

            old_ip != new_ip
        }
    }
}
