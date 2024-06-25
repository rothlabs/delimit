use std::{collections::HashSet, hash::Hash, sync::{Arc, RwLock, Weak}};

use crate::{Meta, NO_POISON};

pub trait React {
    fn clear(&mut self) -> Reactors;
    fn react(&mut self);
}

pub trait ReactLink {
    fn edge(&self) -> &Arc<RwLock<dyn React>>;
    fn meta(&self) -> &Meta;
}

pub trait AddReactor {
    fn add_reactor(&mut self, reactor: &Reactor);
}

pub trait FromReactor {
    fn from_reactor(&self, reactor: Reactor) -> Self;
}

#[derive(Clone)]
pub struct Reactor {
    edge: Weak<RwLock<dyn React>>,
    meta: Meta,
}

impl Reactor {
    pub fn new<E: ReactLink>(link: &E) -> Self { //  + 'static
        Self {
            edge: Arc::downgrade(link.edge()),
            meta: link.meta().clone(),
        }
    }
    pub fn clear(&self) -> Reactors {
        if let Some(edge) = self.edge.upgrade() {
            let mut edge = edge.write().expect(NO_POISON);
            edge.clear()
        } else {
            Reactors::default()
        }
    }
    pub fn react(&self) {
        if let Some(edge) = self.edge.upgrade() {
            let mut edge = edge.write().expect(NO_POISON);
            edge.react();
        }
    }
}

impl PartialEq for Reactor {
    fn eq(&self, other: &Self) -> bool {
        self.meta.id == other.meta.id
    }
}

impl Eq for Reactor {}

impl Hash for Reactor {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.meta.id.hash(state);
    }
}

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
            if rcts.0.len() < 1 {
                reactors.add(reactor);
            } else {
                reactors.0.extend(rcts.0);
            }
        }
        reactors
    }
    pub fn add(&mut self, reactor: &Reactor) {
        self.0.insert(reactor.clone());
    }
}

impl Default for Reactors {
    fn default() -> Self {
        Self(HashSet::new())
    }
}

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