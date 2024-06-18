use std::{
    hash::{Hash, Hasher}, rc::Rc, sync::{Arc, RwLock}
};

use serde::{Serialize, Serializer};

use crate::{Edge, Id, LeafStr, Node, Solve};

#[derive(Clone)]
pub struct Snap {
    pub base: Arc<RwLock<Snapshot>>,
    pub meta: Meta,
}

impl Snap {
    pub fn edge<U: Clone + Serialize + Solve<A, G>, A, G: Clone>(&self, unit: U) -> Edge<U, A, G> {
        Edge::new(self, unit)
    }
    pub fn str(self, unit: &str) -> LeafStr {
        self.edge(unit.to_owned())
    }
}

impl Hash for Snap {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.meta.id.hash(state);
    }
}

impl PartialEq for Snap  {
    fn eq(&self, rhs: &Snap) -> bool {
        self.meta.id == rhs.meta.id
    }
}

impl Eq for Snap {}

impl Serialize for Snap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.meta.serialize(serializer)
    }
}


#[derive(Clone, Serialize)]
pub struct Meta {
    id: Id,
}

//#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Snapshot {
    //pub id: Id,
    //nodes: Vec<Node>,
}

//impl Eq for Snap {}

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
