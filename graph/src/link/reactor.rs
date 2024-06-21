use std::sync::{Weak, RwLock};

use crate::{edge, node, NO_POISON, ROOT};

use super::React;

pub struct Reactor(Weak<RwLock<Box<dyn React<Edge = edge::Reactor>>>>);

impl React for Reactor {
    type Edge = edge::Reactor;
    fn react(&self, vary: <<Self::Edge as edge::React>::Root as node::React>::Vary) {
        let arc = self.0.upgrade().expect(ROOT);
        let root = arc.write().expect(NO_POISON);
        root.react(vary);
    }
}
