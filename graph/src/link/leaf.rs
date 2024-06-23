use std::sync::{Arc, RwLock};

use serde::Serialize;

use crate::{edge, node, FromRoot, FromUnit, Link};

use super::{CloneUnit, Read, Write};

#[derive(derivative::Derivative)]
#[derivative(Clone(bound = ""))]
pub struct Leaf<U>(Link<edge::Leaf<U>>);

impl<U> FromUnit for Leaf<U> {
    type Unit = U;
    fn new(unit: U) -> Self {
        Self(Link::new(unit))
    }
}

impl<U> FromRoot for Leaf<U> {
    type Root = <edge::Leaf<U> as FromRoot>::Root;
    fn from_root(&self, root: &Arc<RwLock<Self::Root>>) -> Self {
        Self(self.0.from_root(root))
    }
}

impl<U> Read for Leaf<U> {
    type Edge = edge::Leaf<U>;
    fn read<F: FnOnce(&<<Self::Edge as edge::Read>::Stem as node::Read>::Unit)>(&self, read: F) {
        self.0.read(read);
    }
}

impl<U> Write for Leaf<U> {
    type Edge = edge::Leaf<U>;
    fn write<F: FnOnce(&mut <<Self::Edge as edge::Read>::Stem as node::Read>::Unit)>(
        &self,
        read: F,
    ) {
        self.0.write(read);
    }
}

impl<U: Clone> CloneUnit for Leaf<U> {
    type Edge = edge::Leaf<U>;
    fn unit(&self) -> <<Self::Edge as edge::Read>::Stem as node::Read>::Unit {
        self.0.unit()
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
