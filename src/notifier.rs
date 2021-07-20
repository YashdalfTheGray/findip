use crate::ip_result::IpResultStorage;

pub trait IPNotifier: IpResultStorage {
    fn notify();
    fn notify_on_change(&self) {
        if self.has_changed() {
            Self::notify()
        }
    }
}
