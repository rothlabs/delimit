use crate::*;

use std::{
    collections::HashSet,
    hash::Hash,
    sync::{Arc, RwLock, Weak},
};

use crate::{Meta, NO_POISON};

pub trait React {
    fn clear(&mut self) -> Reactors;
    fn react(&mut self);
}

pub trait ToReactor {
    fn reactor(&self) -> Reactor;
}

pub trait AddReactor {
    fn add_reactor(&mut self, reactor: Reactor);
}

pub trait WithReactor {
    fn with_reactor(&self, reactor: Reactor) -> Self;
}

pub trait SolverWithReactor {
    type Task;
    type Load;
    fn solver_with_reactor(
        &self,
        reactor: Reactor,
    ) -> Arc<RwLock<dyn Solve<Task = Self::Task, Load = Self::Load>>>;
}

// pub trait SolverWithReactor {
//     type Task;
//     type Load;
//     fn solver_with_reactor(
//         &self,
//         reactor: Reactor,
//     ) -> Arc<RwLock<dyn SolveReact<Self::Task, Self::Load>>>;
// }

#[derive(Clone)]
pub struct Reactor {
    pub item: Weak<RwLock<dyn React>>,
    pub meta: Meta,
}

impl Reactor {
    pub fn clear(&self) -> Reactors {
        // println!("strong_count: {}", Weak::strong_count(&self.item));
        if let Some(item) = self.item.upgrade() {
            let mut item = item.write().expect(NO_POISON);
            item.clear()
        } else {
            Reactors::default()
        }
    }
    pub fn react(&self) {
        if let Some(item) = self.item.upgrade() {
            let mut item = item.write().expect(NO_POISON);
            item.react();
        }
    }
}

impl PartialEq for Reactor {
    fn eq(&self, other: &Self) -> bool {
        Weak::ptr_eq(&self.item, &other.item) && self.meta.id == other.meta.id
    }
}

impl Eq for Reactor {}

impl Hash for Reactor {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.meta.id.hash(state);
    }
}

#[derive(Default)]
pub struct Reactors(HashSet<Reactor>);

impl Reactors {
    // TODO: make method to remove reactors with dropped edges
    pub fn cycle(&mut self) {
        let reactors = self.clear();
        self.0.clear();
        for root in &reactors.0 {
            root.react();
        }
    }
    pub fn clear(&self) -> Reactors {
        let mut reactors = Reactors::default();
        for reactor in &self.0 {
            let rcts = reactor.clear();
            if rcts.0.is_empty() {
                reactors.0.insert(reactor.clone());
            } else {
                reactors.0.extend(rcts.0);
            }
        }
        reactors
    }
    pub fn add(&mut self, reactor: Reactor) {
        self.0.insert(reactor);
    }
}

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
