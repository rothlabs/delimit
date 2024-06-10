use std::{cell::RefCell, rc::Rc, sync::{Arc, Mutex, Weak}};

use serde::{Serializer, Serialize};

use crate::{node::Node, Id};

#[derive(Clone)]
pub struct SnapWeak(pub Weak<Snap>);

#[derive(Clone)]
pub struct SnapWeakMutex(pub Weak<Mutex<Snap>>);

#[derive(Clone)]
pub struct SnapArc(pub Arc<Snap>);
pub struct SnapCell(pub Rc<RefCell<Snap>>);

impl Serialize for SnapCell {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.borrow().id)
    }
}

pub struct Snap {
    id: Id,
    //nodes: Vec<Node>,
}