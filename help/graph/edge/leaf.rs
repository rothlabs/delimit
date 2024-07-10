use std::sync::{Arc, RwLock};

use crate::*;

pub struct Leaf<U> {
    root: Option<Reactor>,
    stem: Arc<RwLock<node::Leaf<U>>>,
    meta: Meta,
}

impl<U: Clone> Leaf<U> {
    pub fn load(&self) -> U {
        let stem = self.stem.read().expect(NO_POISON);
        stem.unit.clone()
    }
}

impl<U> FromUnit for Leaf<U> {
    type Unit = U;
    fn new(unit: U) -> Self {
        Self {
            root: None,
            stem: Arc::new(RwLock::new(node::Leaf::new(unit))),
            meta: Meta::new(),
        }
    }
}

impl<U> WithReactor for Leaf<U> {
    fn with_reactor(&self, reactor: &Reactor) -> Self {
        Self {
            root: Some(reactor.clone()),
            stem: self.stem.clone(),
            meta: self.meta.clone(),
        }
    }
}

impl<U> Reader for Leaf<U> {
    type Unit = U;
    fn reader<F: FnOnce(&Self::Unit)>(&self, read: F) {
        let stem = self.stem.read().expect(NO_POISON);
        read(stem.read());
    }
}

impl<U> Writer for Leaf<U> {
    type Unit = U;
    fn writer<F: FnOnce(&mut Self::Unit)>(&self, write: F) {
        let mut stem = self.stem.write().expect(NO_POISON);
        stem.write(write);
    }
}

// impl<U: Clone> Solve for Leaf<U> {
//     type Load = U;
//     fn solve(&self) -> U {
//         let stem = self.stem.read().expect(NO_POISON);
//         stem.unit.clone()
//     }
// }

impl<U> AddReactor for Leaf<U> {
    fn add_reactor(&mut self, reactor: Reactor) {
        let mut stem = self.stem.write().expect(NO_POISON);
        stem.add_reactor(reactor);
    }
}

impl<U> React for Leaf<U> {
    fn clear(&mut self) -> Reactors {
        if let Some(root) = &self.root {
            root.clear()
        } else {
            Reactors::new()
        }
    }
    fn react(&mut self) {
        if let Some(root) = &self.root {
            root.react();
        }
    }
}

// impl<U: Clone> CloneUnit for Leaf<U> {
//     type Unit = U;
//     fn unit(&self) -> Self::Unit {
//         let stem = self.stem.read().expect(NO_POISON);
//         stem.read().clone()
//     }
// }
