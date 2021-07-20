use std::net::IpAddr;

pub trait IPNotifier {
    fn notify(ip: IpAddr);
}
