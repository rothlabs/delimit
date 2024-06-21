use super::React;

pub struct Reactor(Box<dyn React<Vary = ()>>);

impl React for Reactor {
    type Vary = ();
    fn react(&mut self, vary: Self::Vary) {
        self.0.react(vary);
    }
}
