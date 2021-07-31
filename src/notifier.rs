use std::net::IpAddr;

use crate::errors::IpConflict;

pub trait IpNotifier {
    fn notify_success(&self, ip: IpAddr);
    fn notify_error(&self, err: IpConflict);
}
