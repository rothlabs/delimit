use std::{cell::RefCell, rc::Rc};

use serde::{Serializer, Serialize};

use crate::{node::Node, Id};

pub struct Snap(pub Rc<RefCell<Snapshot>>);

impl Serialize for Snap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.borrow().id)
    }
}

pub struct Snapshot {
    id: Id,
    nodes: Vec<Node>,
}