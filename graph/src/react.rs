use crate::*;

use std::{
    collections::HashSet,
    hash::Hash,
    sync::{Arc, RwLock, Weak},
};

use crate::{Meta, NO_POISON};

pub trait Rebut {
    type Ring;
    fn rebut(&self) -> Self::Ring;
}

pub trait Rebuter {
    type Ring;
    fn rebuter(&mut self) -> Self::Ring;
}

pub trait React {
    fn react(&self);
}

pub trait Reactor {
    fn reactor(&mut self);
}

pub trait AddRoot {
    type Root;
    fn add_root(&mut self, root: Self::Root);
}

pub trait Backed {
    type Back;
    fn backed(&self, back: &Self::Back) -> Self;
}

pub trait ProduceWithBack {
    type Load;
    fn produce_with_back(&self, back: Back) -> Arc<RwLock<dyn Produce<Self::Load> + Send + Sync>>;
}

pub trait ConvertWithBack {
    type Task;
    type Load;
    fn convert_with_back(
        &self,
        back: Back,
    ) -> Arc<RwLock<dyn Convert<Self::Task, Self::Load> + Send + Sync>>;
}

pub trait Cycle {
    fn cycle(&mut self);
}

/// Edge that Rebuts a Ring and reacts.
pub trait Update: Rebut<Ring = Ring> + React {}

/// Node that Rebuts a Ring and mutability reacts.
pub trait Updater: Rebuter<Ring = Ring> + Reactor {}

/// Weakly point to a root edge, the inverse of Link.
/// Should meta be removed?
#[derive(Clone)]
pub struct Root {
    pub edge: Weak<RwLock<dyn Update + Send + Sync>>,
    pub meta: Meta,
}

impl Rebut for Root {
    type Ring = Ring;
    fn rebut(&self) -> Self::Ring {
        // println!("strong_count: {}", Weak::strong_count(&self.item));
        if let Some(edge) = self.edge.upgrade() {
            let edge = edge.read().expect(NO_POISON);
            edge.rebut()
        } else {
            Ring::new()
        }
    }
}

impl React for Root {
    fn react(&self) {
        if let Some(edge) = self.edge.upgrade() {
            let edge = edge.read().expect(NO_POISON);
            edge.react();
        }
    }
}

impl PartialEq for Root {
    fn eq(&self, other: &Self) -> bool {
        Weak::ptr_eq(&self.edge, &other.edge) && self.meta.id == other.meta.id
    }
}

impl Eq for Root {}

impl Hash for Root {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.meta.id.hash(state);
    }
}

/// Weakly point to the back of a node as Updater.
#[derive(Clone)]
pub struct Back {
    pub node: Weak<RwLock<dyn Updater + Send + Sync + 'static>>,
    // pub meta: Meta,
}

impl Rebut for Back {
    type Ring = Ring;
    fn rebut(&self) -> Self::Ring {
        // println!("strong_count: {}", Weak::strong_count(&self.item));
        if let Some(node) = self.node.upgrade() {
            let mut node = node.write().expect(NO_POISON);
            node.rebuter()
        } else {
            Ring::new()
        }
    }
}

impl React for Back {
    fn react(&self) {
        if let Some(node) = self.node.upgrade() {
            let mut node = node.write().expect(NO_POISON);
            node.reactor();
        }
    }
}

/// Points to many root edges, each pointing to a node back.
#[derive(Default)]
pub struct Ring {
    roots: HashSet<Root>,
}

impl Ring {
    pub fn new() -> Self {
        Self::default()
    }
    // TODO: make method to remove reactors with dropped edges
}

impl Cycle for Ring {
    fn cycle(&mut self) {
        let ring = self.rebut();
        self.roots.clear();
        for root in &ring.roots {
            root.react();
        }
    }
}

impl Rebut for Ring {
    type Ring = Self;
    fn rebut(&self) -> Self::Ring {
        let mut result = Ring::new();
        for root in &self.roots {
            let ring = root.rebut();
            if ring.roots.is_empty() {
                result.roots.insert(root.clone());
            } else {
                result.roots.extend(ring.roots);
            }
        }
        result
    }
}

impl AddRoot for Ring {
    type Root = Root;
    fn add_root(&mut self, root: Self::Root) {
        self.roots.insert(root);
    }
}
