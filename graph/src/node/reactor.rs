use crate::React;

pub struct Reactor(Box<dyn React>);

impl React for Reactor {
    fn react(&mut self) {
        self.0.react();
    }
}
