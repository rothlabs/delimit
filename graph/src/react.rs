use crate::*;

use std::{
    collections::HashSet,
    hash::Hash,
    sync::{Arc, RwLock, Weak},
};

use crate::{Meta, NO_POISON};

pub trait Rebuter {
    type Ring;
    fn rebut(&self) -> Self::Ring;
}

pub trait Rebut {
    type Ring;
    fn rebuter(&mut self) -> Self::Ring;
}

pub trait Reactor {
    fn reactor(&self);
}

pub trait React {
    fn react(&mut self);
}

pub trait AddRoot {
    type Root;
    fn add_root(&mut self, root: Self::Root);
}

pub trait Backed {
    type Back;
    fn backed(&self, back: &Self::Back) -> Self;
}

pub trait PloyWithBack {
    type Load;
    fn ploy_with_back(&self, back: Back) -> Arc<RwLock<dyn Produce<Self::Load> + Send + Sync>>;
}

pub trait PlanWithBack {
    type Task;
    type Load;
    fn plan_with_back(
        &self,
        back: Back,
    ) -> Arc<RwLock<dyn Convert<Self::Task, Self::Load> + Send + Sync>>;
}

pub trait Cycle {
    fn cycle(&mut self);
}

/// Edge that Rebuts a Ring and reacts.
pub trait Updater: Rebuter<Ring = Ring> + Reactor {}

/// Node that Rebuts a Ring and mutably reacts.
pub trait Update: Rebut<Ring = Ring> + React {}

/// Weakly point to a root edge, the inverse of Link.
/// Should meta be removed?
#[derive(Clone)]
pub struct Root {
    pub edge: Weak<RwLock<dyn Updater + Send + Sync>>,
    pub meta: Meta,
}

impl Rebuter for Root {
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

impl Reactor for Root {
    fn reactor(&self) {
        if let Some(edge) = self.edge.upgrade() {
            let edge = edge.read().expect(NO_POISON);
            edge.reactor();
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
    pub node: Weak<RwLock<dyn Update + Send + Sync + 'static>>,
    // pub meta: Meta,
}

impl Rebuter for Back {
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

impl Reactor for Back {
    fn reactor(&self) {
        if let Some(node) = self.node.upgrade() {
            let mut node = node.write().expect(NO_POISON);
            node.react();
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
            root.reactor();
        }
    }
}

impl Rebuter for Ring {
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
