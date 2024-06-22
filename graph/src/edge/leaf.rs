use std::sync::{Arc, RwLock};

use crate::node::{self, New, Reactor};
use crate::{Edge, Meta};


pub struct Leaf<U>(Edge<Reactor, node::Leaf<U>>);

impl<U> super::New for Leaf<U> {
    type Unit = U;
    fn new(unit: U) -> Self {
        Self (
            Edge {
                root: None,
                stem: Arc::new(RwLock::new(node::Leaf::new(unit))),
                meta: Meta::new(),
            }
        )
    }
}