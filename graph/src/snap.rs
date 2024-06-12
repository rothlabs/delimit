use std::{cell::RefCell, rc::Rc, sync::{Arc, Mutex, Weak}};

use serde::{Serializer, Serialize};

use crate::{Id, Node};

// #[derive(Clone)]
// pub struct SnapWeak(pub Weak<Snap>);

// #[derive(Clone)]
// pub struct SnapWeakMutex(pub Weak<Mutex<Snap>>);

// #[derive(Clone, PartialEq, Hash)]
// pub struct Snap(pub Node<Unit>);

// impl Eq for Snap {}
//pub struct SnapCell(pub Rc<RefCell<Snap>>);

// impl Serialize for SnapCell {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         serializer.serialize_str(&self.0.borrow().id)
//     }
// }

#[derive(Clone, PartialEq, Hash)]
pub struct Snap {
    pub id: Id,
    //nodes: Vec<Node>,
}

//impl Eq for Snap {}