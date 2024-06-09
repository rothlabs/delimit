use std::rc::Rc;

use crate::node::Id;

pub struct Snap {
    nodes: Vec<Rc<Id>>,
}