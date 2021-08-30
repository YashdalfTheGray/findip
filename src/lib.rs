use log::{error, info};

use crate::ip_query::IpQueryParams;

pub mod config_file;
pub mod errors;
pub mod ip_query;
pub mod ip_result;
pub mod notifier;
pub mod sdk;
pub mod utils;

pub fn schedule_ip_notification(config: config_file::ConfigFile) {
    match ip_query::run_ip_query(IpQueryParams {
        services: config.services.clone(),
    }) {
        Ok(ip_result) => {
            info!("{:?}", ip_result);
        }
        Err(e) => {
            error!("{:?}", e);
        }
    };
}
