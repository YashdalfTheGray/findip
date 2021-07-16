pub trait IPNotifier {
    fn notify();
    fn notify_on_change(should_notify: bool) {
        if should_notify {
            Self::notify();
        }
    }
}
