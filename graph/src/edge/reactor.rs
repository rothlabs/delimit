use crate::node;

use super::React;

pub struct Reactor(Box<dyn React<Root = node::Reactor>>);

impl React for Reactor {
    type Root = node::Reactor;
    fn react(&self, vary: <Self::Root as node::React>::Vary) {
        self.0.react(vary);
    }
}