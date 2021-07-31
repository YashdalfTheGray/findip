use std::net::IpAddr;

use crate::errors::IpError;

pub trait IpNotifier {
    fn notify_success(&self, ip: IpAddr);
    fn notify_error(&self, err: IpError);
}
