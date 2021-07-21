use std::net::IpAddr;

pub trait IpNotifier {
    fn notify(ip: IpAddr);
}
