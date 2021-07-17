use std::{error::Error, fmt, net::IpAddr, sync::Mutex};

use chrono::{DateTime, Utc};

struct IPResult {
    pub ip: IpAddr,
    pub checked_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct IPError {}

impl fmt::Display for IPError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IP Address not found.",)
    }
}

impl Error for IPError {}

impl IPError {
    pub fn new() -> Self {
        IPError {}
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

    pub fn add_result(self, ip: IpAddr, checked_at: DateTime<Utc>) {
        let mut locked_results = self.results.lock().unwrap();
        if (*locked_results).len() >= 2 {
            (*locked_results).truncate(1);
        }
        (*locked_results).push(IPResult { ip, checked_at })
    }

    pub fn get_latest_ip(self) -> Result<IpAddr, IPError> {
        let locked_results = self.results.lock().unwrap();

        if (*locked_results).len() == 0 {
            Err(IPError::new())
        } else {
            Ok((*locked_results)[0].ip)
        }
    }

    pub fn has_changed(self) -> bool {
        let locked_results = self.results.lock().unwrap();
        if (*locked_results).len() < 2 {
            false
        } else {
            let old_ip = (*locked_results)[1].ip;
            let new_ip = (*locked_results)[0].ip;

            old_ip != new_ip
        }
    }
}
