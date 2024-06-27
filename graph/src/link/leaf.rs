use serde::Serialize;

use crate::*;

#[derive(derivative::Derivative)]
#[derivative(Clone(bound = ""))]
pub struct Leaf<U>(Link<edge::Leaf<U>>);

impl<U> FromUnit for Leaf<U> {
    type Unit = U;
    fn from_unit(unit: U) -> Self {
        Self(Link::from_unit(unit))
    }
}

impl<U> FromReactor for Leaf<U> {
    fn from_reactor(&self, root: Reactor) -> Self {
        Self(self.0.from_reactor(root))
    }
}

impl<U> Reader for Leaf<U> {
    type Unit = U;
    fn reader<F: FnOnce(&U)>(&self, read: F) {
        self.0.reader(read);
    }
}

impl<U> Writer for Leaf<U> {
    type Unit = U;
    fn writer<F: FnOnce(&mut U)>(&self, read: F) {
        self.0.writer(read);
    }
}

impl<U: Clone> CloneUnit for Leaf<U> {
    type Unit = U;
    fn unit(&self) -> U {
        self.0.unit()
    }
}

impl<U> PartialEq for Leaf<U> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<U> Serialize for Leaf<U> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

// impl<U> Clone for Leaf<U> {
//     fn clone(&self) -> Self {
//         Self(self.0.clone())
//     }
// }

// pub struct Leaf<U> {
//     edge: Arc<RwLock<edge::Leaf<U>>>,
//     meta: Meta,
// }

// impl<U> self::New for Leaf<U> {
//     type Unit = U;
//     fn new(unit: U) -> Self {
//         Self {
//             edge: Arc::new(RwLock::new(edge::Leaf::new(unit))),
//             meta: Meta::new(),
//         }
//     }
// }

// impl<U> Clone for Leaf<U> {
//     fn clone(&self) -> Self {
//         Self {
//             edge: self.edge.clone(),
//             meta: self.meta.clone(),
//         }
//     }
// }

// impl<U> Serialize for Leaf<U> {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//         where
//             S: serde::Serializer {
//         self.meta.serialize(serializer)
//     }
// }

// impl<U> New for Leaf<U> //{
//     where
//         edge::Leaf<U>: edge::New,
//         //U: edge::New,
//     {
//         type Unit = U; //::Unit;
//         fn new(unit: U) -> Self {
//             Self {
//                 edge: Arc::new(RwLock::new(<edge::Leaf<U> as edge::New>::new(unit))),
//                 meta: Meta::new(),
//             }
//         }
//     }
