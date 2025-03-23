pub struct Event<T>(Box<dyn FnMut() -> T>);

impl<T> Event<T> {
    pub fn dispatch(&mut self) -> T {
        (self.0)()
    }
}
