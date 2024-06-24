use std::sync::{RwLock, Weak};

use super::React;

pub struct Reactor(Box<dyn React>);

impl React for Reactor {
    fn react(&self) {
        self.0.react();
    }
}


