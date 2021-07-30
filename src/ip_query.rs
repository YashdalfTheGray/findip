use std::{
    collections::HashSet,
    hash::Hash,
    net::{IpAddr, Ipv4Addr},
};

use crate::ip_error::IpError;

pub struct IpQueryParams {
    pub services: Vec<String>,
}

pub fn run_ip_query(params: IpQueryParams) -> Result<IpAddr, IpError> {
    let _ips: Vec<String> = params
        .services
        .iter()
        .map(|service| match reqwest::blocking::get(service) {
            Ok(response) => response.text().unwrap_or("".to_string()),
            Err(_) => "".to_string(),
        })
        .filter(|maybe_ip| !maybe_ip.is_empty())
        .collect();

    Ok(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
}

fn uniq<T: Eq + Hash>(mut v: Vec<T>) -> Vec<T> {
    let set: HashSet<T> = v.drain(..).collect();
    v.extend(set.into_iter());
    v
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
}
