pub trait IPNotifier {
    type ReturnType;

    fn notify() -> Self::ReturnType;
    fn notify_on_change() -> Self::ReturnType {
        Self::notify()
    }
}
