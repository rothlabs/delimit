use std::sync::{Arc, RwLock};

use serde::Serialize;

use crate::*;

#[derive(Clone)]
pub struct Leaf<U> {
    edge: Arc<RwLock<edge::Leaf<U>>>,
    meta: Meta,
}

impl<U> PartialEq for Leaf<U> {
    fn eq(&self, other: &Self) -> bool {
        Arc::<RwLock<edge::Leaf<U>>>::ptr_eq(&self.edge, &other.edge) && self.meta == other.meta
    }
}

impl<U> FromUnit for Leaf<U> {
    type Unit = U;
    fn new(unit: U) -> Self {
        Self {
            edge: Arc::new(RwLock::new(edge::Leaf::new(unit))),
            meta: Meta::new(),
        }
    }
}

impl WithReactor for Leaf<String> {
    fn with_reactor(&self, reactor: &Reactor) -> Self {
        let edge = self.edge.read().expect(NO_POISON);
        Self {
            edge: Arc::new(RwLock::new(edge.with_reactor(reactor))),
            meta: self.meta.clone(),
        }
    }
}

impl<U> ToReactor for Leaf<U>
where
    U: 'static,
{
    fn reactor(&self) -> Reactor {
        let edge = self.edge.clone() as Arc<RwLock<dyn React>>;
        Reactor {
            item: Arc::downgrade(&edge),
            meta: self.meta.clone(),
        }
    }
}

impl<U> Reader for Leaf<U> 
where 
    U: 'static,
{
    type Unit = U;
    fn reader<F: FnOnce(&U)>(&self, read: F) {
        let mut edge = self.edge.write().expect(NO_POISON);
        edge.reader(read);
        let reactor = self.reactor();
        edge.add_reactor(reactor);
    }
}

impl<U> Writer for Leaf<U> {
    type Unit = U;
    fn writer<F: FnOnce(&mut U)>(&self, write: F) {
        let edge = self.edge.read().expect(NO_POISON);
        edge.writer(write);
    }
}

impl<U: Clone> CloneUnit for Leaf<U> {
    type Unit = U;
    fn unit(&self) -> U {
        let edge = self.edge.read().expect(NO_POISON);
        edge.unit()
    }
}

impl<U> Serialize for  Leaf<U>  {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.meta.serialize(serializer)
    }
}

pub trait ToLeaf<T> {
    fn leaf(&self) -> Leaf<T>;
}

impl ToLeaf<String> for str {
    fn leaf(&self) -> Leaf<String> {
        Leaf::new(self.to_owned())
    }
}

impl<T: GraphString> ToLeaf<String> for T {
    fn leaf(&self) -> Leaf<String> {
        self.string().into_leaf()
    }
}

pub trait IntoLeaf<T> {
    fn into_leaf(self) -> Leaf<T>;
}

impl<T> IntoLeaf<T> for T {
    fn into_leaf(self) -> Leaf<T> {
        Leaf::new(self)
    }
}

// impl<U> PartialEq for Leaf<U> {
//     fn eq(&self, other: &Self) -> bool {
//         self.0 == other.0
//     }
// }

// impl<U> Serialize for Leaf<U> {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         self.0.serialize(serializer)
//     }
// }

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
