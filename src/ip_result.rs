use std::{net::IpAddr, sync::Mutex};

use chrono::{DateTime, Utc};

use crate::{
    ip_error::IpError,
    ip_query::{run_ip_query, IpQueryParams},
    notifier::IpNotifier,
};

pub trait IpResultStorage {
    type ErrorType;

    fn add_result(&self, ip: IpAddr, checked_at: DateTime<Utc>);
    fn get_latest_ip(&self) -> Result<IpAddr, Self::ErrorType>;
    fn ip_has_changed(&self) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
struct IpResult {
    pub ip: IpAddr,
    pub checked_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct IpResults {
    only_notify_on_change: bool,
    results: Mutex<Vec<IpResult>>,
}

impl IpResults {
    pub fn new(only_notify_on_change: Option<bool>) -> IpResults {
        IpResults {
            only_notify_on_change: only_notify_on_change.unwrap_or(false),
            results: Mutex::new(Vec::new()),
        }
    }

    pub fn query_ip<N>(&self, notifier: N)
    where
        N: IpNotifier,
    {
        let params = IpQueryParams {
            services: vec!["test".to_string()],
        };

        match run_ip_query(params) {
            Ok(ip) => {
                self.add_result(ip, Utc::now());
                if self.only_notify_on_change {
                    if self.ip_has_changed() {
                        notifier.notify_success(ip);
                    }
                } else {
                    notifier.notify_success(ip);
                }
            }
            Err(e) => {
                notifier.notify_error(e);
            }
        };
    }
}

impl IpResultStorage for IpResults {
    type ErrorType = IpError;

    fn add_result(&self, ip: IpAddr, checked_at: DateTime<Utc>) {
        let mut locked_results = self.results.lock().unwrap();
        if (*locked_results).len() >= 2 {
            (*locked_results).truncate(1);
        }
        (*locked_results).push(IpResult { ip, checked_at })
    }

    fn get_latest_ip(&self) -> Result<IpAddr, IpError> {
        let locked_results = self.results.lock().unwrap();

        if (*locked_results).len() == 0 {
            Err(IpError::new())
        } else {
            Ok((*locked_results)[0].ip)
        }
    }

    fn ip_has_changed(&self) -> bool {
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

#[cfg(test)]
mod tests {

    use std::{error::Error, fmt, net::Ipv4Addr};

    use super::*;

    #[derive(Debug, Clone)]
    struct UnexpectedOutputError {}

    impl fmt::Display for UnexpectedOutputError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "The output of the test was unexpected.",)
        }
    }

    impl Error for UnexpectedOutputError {}

    #[test]
    fn test_new_ip_results_with_defaults() {
        let ip_results = IpResults::new(None);
        assert!(ip_results.only_notify_on_change == false);
    }

    #[test]
    fn test_new_ip_results_with_a_value() {
        let ip_results = IpResults::new(Some(true));
        assert!(ip_results.only_notify_on_change == true);
    }

    #[test]
    fn test_add_result() {
        let results = IpResults::new(Some(false));

        results.add_result(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), Utc::now());
        assert_eq!(
            results.get_latest_ip().unwrap(),
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))
        );
    }

    #[test]
    fn test_get_latest_ip_when_no_ips() -> Result<(), UnexpectedOutputError> {
        let results = IpResults::new(Some(false));

        match results.get_latest_ip() {
            Ok(_) => Err(UnexpectedOutputError {}),
            Err(_) => Ok(()),
        }
    }

    #[test]
    fn test_get_latest_ip_when_only_one_ip() -> Result<(), UnexpectedOutputError> {
        let results = IpResults::new(Some(false));

        results.add_result(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), Utc::now());
        match results.get_latest_ip() {
            Ok(ip) => {
                assert_eq!(ip, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
                Ok(())
            }
            Err(_) => Err(UnexpectedOutputError {}),
        }
    }

    #[test]
    fn test_ip_has_changed_without_ips() {
        let results = IpResults::new(Some(false));
        assert!(results.ip_has_changed() == false);
    }

    #[test]
    fn test_ip_has_changed_with_one_ip() {
        let results = IpResults::new(Some(false));
        results.add_result(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), Utc::now());

        assert!(results.ip_has_changed() == true);
    }

    #[test]
    fn test_ip_has_changed_with_two_different_ips() {
        let results = IpResults::new(Some(false));
        results.add_result(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), Utc::now());
        results.add_result(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 2)), Utc::now());

        assert!(results.ip_has_changed() == true);
    }

    #[test]
    fn test_ip_has_changed_with_two_of_the_same_ips() {
        let results = IpResults::new(Some(false));
        results.add_result(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), Utc::now());
        results.add_result(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), Utc::now());

        assert!(results.ip_has_changed() == false);
    }
}
