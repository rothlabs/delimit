use std::sync::{Arc, RwLock};

use crate::*;

#[derive(Clone)]
pub struct Leaf<U>(Edge<node::Leaf<U>>);

impl<U> FromUnit for Leaf<U> {
    type Unit = U;
    fn new(unit: U) -> Self {
        Self(Edge {
            root: None,
            stem: Arc::new(RwLock::new(node::Leaf::new(unit))),
            meta: Meta::new(),
        })
    }
}

impl<U> WithReactor for Leaf<U> {
    fn with_reactor(&self, reactor: Reactor) -> Self {
        Self(self.0.with_reactor(reactor))
    }
}

impl<U> Reader for Leaf<U> {
    type Unit = U;
    fn reader<F: FnOnce(&Self::Unit)>(&self, read: F) {
        self.0.reader(read);
    }
}

impl<U> Writer for Leaf<U> {
    type Unit = U;
    fn writer<F: FnOnce(&mut Self::Unit)>(&self, read: F) {
        self.0.writer(read);
    }
}

impl<U: Clone> CloneUnit for Leaf<U> {
    type Unit = U;
    fn unit(&self) -> Self::Unit {
        self.0.unit()
    }
}

impl<U> AddReactor for Leaf<U> {
    fn add_reactor(&mut self, reactor: Reactor) {
        self.0.add_reactor(reactor);
    }
}

impl<U> React for Leaf<U> {
    fn clear(&mut self) -> Reactors {
        self.0.clear()
    }
    fn react(&mut self) {
        self.0.react()
    }
}
