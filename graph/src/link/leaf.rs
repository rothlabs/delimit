use std::sync::{Arc, RwLock};

use serde::Serialize;

use crate::*;

#[derive(Clone)]
pub struct Leaf<L> {
    edge: Arc<RwLock<edge::Leaf<L>>>,
    meta: Meta,
}

impl<L: Clone> Leaf<L> {
    pub fn load(&self) -> L {
        let edge = self.edge.read().expect(NO_POISON);
        edge.load()
    }
}

impl<L> PartialEq for Leaf<L> {
    fn eq(&self, other: &Self) -> bool {
        Arc::<RwLock<edge::Leaf<L>>>::ptr_eq(&self.edge, &other.edge) && self.meta == other.meta
    }
}

impl<L> FromUnit for Leaf<L> {
    type Unit = L;
    fn new(unit: L) -> Self {
        Self {
            edge: Arc::new(RwLock::new(edge::Leaf::new(unit))),
            meta: Meta::new(),
        }
    }
}

impl<L> WithReactor for Leaf<L> {
    fn with_reactor(&self, reactor: &Reactor) -> Self {
        let edge = self.edge.read().expect(NO_POISON);
        Self {
            edge: Arc::new(RwLock::new(edge.with_reactor(reactor))),
            meta: self.meta.clone(),
        }
    }
}

impl<L> ToReactor for Leaf<L>
where
    L: 'static,
{
    fn reactor(&self) -> Reactor {
        let edge = self.edge.clone() as Arc<RwLock<dyn React>>;
        Reactor {
            item: Arc::downgrade(&edge),
            meta: self.meta.clone(),
        }
    }
}

impl<L> Reader for Leaf<L>
where
    L: 'static,
{
    type Unit = L;
    fn reader<F: FnOnce(&Self::Unit)>(&self, read: F) {
        let mut edge = self.edge.write().expect(NO_POISON);
        edge.reader(read);
        let reactor = self.reactor();
        edge.add_reactor(reactor);
    }
}

impl<L> Writer for Leaf<L> {
    type Unit = L;
    fn writer<F: FnOnce(&mut L)>(&self, write: F) {
        let edge = self.edge.read().expect(NO_POISON);
        edge.writer(write);
    }
}

impl<L> Serialize for Leaf<L> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.meta.serialize(serializer)
    }
}

pub trait ToLeaf<L> {
    fn leaf(&self) -> Leaf<L>;
}

impl ToLeaf<String> for str {
    fn leaf(&self) -> Leaf<String> {
        Leaf::new(self.to_owned())
    }
}

pub trait IntoLeaf<L> {
    fn into_leaf(self) -> Leaf<L>;
}

impl<L> IntoLeaf<L> for L {
    fn into_leaf(self) -> Leaf<L> {
        Leaf::new(self)
    }
}

// impl WithReactor for Leaf<String> {
//     fn with_reactor(&self, reactor: &Reactor) -> Self {
//         let edge = self.edge.read().expect(NO_POISON);
//         Self {
//             edge: Arc::new(RwLock::new(edge.with_reactor(reactor))),
//             meta: self.meta.clone(),
//         }
//     }
// }

// impl<U: Clone> Solve for Leaf<U> {
//     type Load = U;
//     fn solve(&self) -> U {
//         let edge = self.edge.read().expect(NO_POISON);
//         edge.solve()
//     }
// }

// impl<U: Clone> CloneUnit for Leaf<U> {
//     type Unit = U;
//     fn unit(&self) -> U {
//         let edge = self.edge.read().expect(NO_POISON);
//         edge.unit()
//     }
// }

// impl<T: GraphString> ToLeaf<String> for T {
//     fn leaf(&self) -> Leaf<String> {
//         self.string().into_leaf()
//     }
// }

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
