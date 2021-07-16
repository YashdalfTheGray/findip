use std::{net::IpAddr, sync::Mutex};

use chrono::{DateTime, Utc};

struct IPResult {
    pub ip: IpAddr,
    pub checked_at: DateTime<Utc>,
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
}
