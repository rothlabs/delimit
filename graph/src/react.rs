use crate::*;

use std::{
    collections::HashSet,
    hash::Hash,
    sync::{Arc, RwLock, Weak},
};

use crate::{Meta, NO_POISON};

pub trait Event {
    type Root;
    fn event(&self) -> Self::Root;
}

pub trait EventMut {
    type Roots;
    fn event_mut(&mut self) -> Self::Roots;
}

pub trait React {
    fn react(&self);
}

pub trait ReactMut {
    fn react_mut(&mut self);
}

pub trait ToRootEdge {
    fn reactor(&self) -> RootEdge;
}

pub trait AddRoot {
    type Root;
    fn add_root(&mut self, reactor: Self::Root);
}

pub trait WithRoot {
    type Root;
    fn with_root(&self, root: &Self::Root) -> Self;
}

pub trait SolverWithRoot {
    type Load;
    fn solver_with_root(
        &self,
        root: Root,
    ) -> Arc<RwLock<dyn Formula<Self::Load> + Send + Sync>>;
}

pub trait TaskerWithRoot {
    type Task;
    type Load;
    fn tasker_with_root(
        &self,
        root: Root,
    ) -> Arc<RwLock<dyn TaskShare<Self::Task, Self::Load> + Send + Sync>>;
}

pub trait Cycle {
    fn cycle(&mut self);
}

pub trait EventReact: Event<Root = Ring> + React {} // + Send + Sync {}

pub trait EventReactMut: EventMut<Roots = Ring> + ReactMut {} //  + Send + Sync {}

/// Weakly point to a root edge, the inverse of Link.
/// Should meta be removed?
#[derive(Clone)]
pub struct RootEdge {
    pub item: Weak<RwLock<dyn EventReact + Send + Sync + 'static>>,
    pub meta: Meta,
}

impl Event for RootEdge {
    type Root = Ring;
    fn event(&self) -> Self::Root {
        // println!("strong_count: {}", Weak::strong_count(&self.item));
        if let Some(item) = self.item.upgrade() {
            let item = item.read().expect(NO_POISON);
            item.event()
        } else {
            Ring::new()
        }
    }
}

impl React for RootEdge {
    fn react(&self) {
        if let Some(item) = self.item.upgrade() {
            let item = item.read().expect(NO_POISON);
            item.react();
        }
    }
}

impl PartialEq for RootEdge {
    fn eq(&self, other: &Self) -> bool {
        Weak::ptr_eq(&self.item, &other.item) && self.meta.id == other.meta.id
    }
}

impl Eq for RootEdge {}

impl Hash for RootEdge {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.meta.id.hash(state);
    }
}

/// Weakly point to a root node as event handler and reactor.
#[derive(Clone)]
pub struct Root {
    pub item: Weak<RwLock<dyn EventReactMut + Send + Sync + 'static>>,
    // pub meta: Meta,
}

impl Event for Root {
    type Root = Ring;
    fn event(&self) -> Self::Root {
        // println!("strong_count: {}", Weak::strong_count(&self.item));
        if let Some(item) = self.item.upgrade() {
            let mut item = item.write().expect(NO_POISON);
            item.event_mut()
        } else {
            Ring::new()
        }
    }
}

impl React for Root {
    fn react(&self) {
        if let Some(item) = self.item.upgrade() {
            let mut item = item.write().expect(NO_POISON);
            item.react_mut();
        }
    }
}

/// Points to many root edges, each pointing to a root node.
#[derive(Default)]
pub struct Ring(HashSet<RootEdge>);

impl Ring {
    pub fn new() -> Self {
        Self::default()
    }
    // TODO: make method to remove reactors with dropped edges
}

impl Cycle for Ring {
    fn cycle(&mut self) {
        let reactors = self.event();
        self.0.clear();
        for root in &reactors.0 {
            root.react();
        }
    }
}

impl Event for Ring {
    type Root = Self;
    fn event(&self) -> Self::Root {
        let mut reactors = Ring::new();
        for root_edge in &self.0 {
            let rcts = root_edge.event();
            if rcts.0.is_empty() {
                reactors.0.insert(root_edge.clone());
            } else {
                reactors.0.extend(rcts.0);
            }
        }
        reactors
    }
}

impl AddRoot for Ring {
    type Root = RootEdge;
    fn add_root(&mut self, reactor: Self::Root) {
        self.0.insert(reactor);
    }
}

// impl React for Reactors {
//     fn react(&self) {

//     }
// }

// impl Default for Reactor {
//     fn default() -> Self {
//         Self {
//             item
//         }
//     }
// }

// impl Reactor {
//     pub fn clear(&self) -> Reactors {
//         // println!("strong_count: {}", Weak::strong_count(&self.item));
//         if let Some(item) = self.item.upgrade() {
//             let mut item = item.write().expect(NO_POISON);
//             item.clear()
//         } else {
//             Reactors::new()
//         }
//     }
//     pub fn react(&self) {
//         if let Some(item) = self.item.upgrade() {
//             let mut item = item.write().expect(NO_POISON);
//             item.react();
//         }
//     }
// }

// impl Reactors {
//     pub fn new() -> Self {
//         Self::default()
//     }
//     // TODO: make method to remove reactors with dropped edges
//     pub fn cycle(&mut self) {
//         let reactors = self.clear();
//         self.0.clear();
//         for root in &reactors.0 {
//             root.react();
//         }
//     }
//     // pub fn clear(&self) -> Reactors {
//     //     let mut reactors = Reactors::new();
//     //     for reactor in &self.0 {
//     //         let rcts = reactor.clear();
//     //         if rcts.0.is_empty() {
//     //             reactors.0.insert(reactor.clone());
//     //         } else {
//     //             reactors.0.extend(rcts.0);
//     //         }
//     //     }
//     //     reactors
//     // }
//     // pub fn add(&mut self, reactor: Reactor) {
//     //     self.0.insert(reactor);
//     // }
// }

// impl Default for Reactors {
//     fn default() -> Self {
//         Self(HashSet::new())
//     }
// }

// pub fn add<T: ToReactor>(&mut self, link: &T) {
//     self.0.insert(link.reactor());
// }

// pub trait AddReactor {
//     fn add_reactor<T: ToReactor>(&mut self, link: &T);
// }

// impl AsReactor for Reactor {
//     fn as_reactor(&self) -> Reactor {
//         self
//     }
// }

// pub fn new<E: ToReactor>(link: &E) -> Self { //  + 'static
//     Self {
//         edge: Arc::downgrade(link.edge()),
//         meta: link.meta().clone(),
//     }
// }

// pub fn root(&self) -> bool {
//     if let Some(edge) = self.edge.upgrade() {
//         let mut edge = edge.write().expect(NO_POISON);
//         edge.clear();
//         edge.root()
//     } else {
//         false
//     }
// }

//let mut roots = Vec::from_iter(reactors.0);

// pub fn extend(&mut self, reactors: Reactors) {
//     self.0.extend(reactors.0);
// }

// pub fn new<R: React + 'static>(link: &Arc<RwLock<R>>) -> Self {
//     let link = Arc::downgrade(&link);
//     Self{
//         link,

//     }
// }
