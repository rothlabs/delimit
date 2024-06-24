use std::sync::{RwLock, Weak};

use crate::{NO_POISON, ROOT};

use super::React;

pub struct Reactor(Weak<RwLock<Box<dyn React>>>);

impl React for Reactor {
    fn react(&self) {
        let arc = self.0.upgrade().expect(ROOT);
        let root = arc.write().expect(NO_POISON);
        root.react();
    }
}
