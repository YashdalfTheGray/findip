use std::{
    hash::Hash,
    net::{IpAddr, Ipv4Addr},
};

use indexmap::IndexSet;

use crate::errors::{ErrorReason, IpError};

pub struct IpQueryParams {
    pub services: Vec<String>,
}

pub fn run_ip_query(params: IpQueryParams) -> Result<IpAddr, IpError> {
    let ips: Vec<String> = params
        .services
        .iter()
        .map(|service| match reqwest::blocking::get(service) {
            Ok(response) => response.text().unwrap_or("".to_string()),
            Err(_) => "".to_string(),
        })
        .filter(|maybe_ip| !maybe_ip.is_empty())
        .collect();

    let uniq_ips: Vec<String> = uniq(ips.clone());

    if uniq_ips.len() > 1 {
        return Err(IpError::new(ErrorReason::IpConflict(ips.clone())));
    }

    Ok(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
}

fn uniq<T: Eq + Hash>(mut v: Vec<T>) -> Vec<T> {
    let set: IndexSet<T> = v.drain(..).collect();
    set.into_iter().collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_uniq_returns_vec_without_duplicates() {
        let v = vec![1, 1, 2, 3, 4, 5, 5, 6, 6, 6, 6, 6];
        let result = uniq(v);
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_uniq_returns_vec_as_is_when_no_duplicates() {
        let v = vec![1, 2, 3, 4, 5, 6];
        let result = uniq(v);
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_uniq_returns_vec_as_is_when_only_one_element() {
        let v = vec![1];
        let result = uniq(v);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_uniq_returns_vec_as_is_when_empty() {
        let v: Vec<i32> = vec![];
        let result = uniq(v);
        assert_eq!(result, Vec::<i32>::new());
    }
}
