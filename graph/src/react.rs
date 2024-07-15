use crate::*;

use std::{
    collections::HashSet,
    hash::Hash,
    sync::{Arc, RwLock, Weak},
};

use crate::{Meta, NO_POISON};

pub trait Rebuter {
    fn rebut(&self) -> Ring;
}

pub trait Rebut {
    fn rebut(&mut self) -> Ring;
}

pub trait Reactor {
    fn react(&self, meta: &Meta);
}

pub trait React {
    fn react(&mut self, meta: &Meta);
}

pub trait AddRoot {
    fn add_root(&mut self, root: Root);
}

pub trait RootAdder {
    fn add_root(&self, root: Root);
}

pub trait Backed {
    fn backed(&self, back: &Back) -> Self;
}

pub trait ToPloy {
    type Load;
    fn ploy(&self) -> Arc<RwLock<Box<dyn Produce<Self::Load> + Send + Sync>>>;
}

pub trait PloyWithBack {
    type Load;
    fn ploy_with_back(&self, back: Back)
        -> Arc<RwLock<Box<dyn Produce<Self::Load> + Send + Sync>>>;
}

type PlanEdge<T, L> = Arc<RwLock<Box<dyn Convert<T, L> + Send + Sync>>>;

pub trait ToPlan {
    type Task;
    type Load;
    fn plan(&self) -> PlanEdge<Self::Task, Self::Load>;
}

pub trait PlanWithBack {
    type Task;
    type Load;
    fn plan_with_back(&self, back: Back) -> PlanEdge<Self::Task, Self::Load>;
}

pub trait ToUpdater {
    fn updater(&self) -> Weak<RwLock<dyn Updater + Send + Sync>>;
}

/// Edge that Rebuts a Ring and reacts.
pub trait Updater: Rebuter + Reactor {}

/// Node that Rebuts a Ring and mutably reacts.
pub trait Update: Rebut + React {}

/// Weakly point to a root edge, the inverse of Link.
/// Should meta be removed?
#[derive(Clone)]
pub struct Root {
    pub edge: Weak<RwLock<dyn Updater + Send + Sync>>,
    pub meta: Meta,
}

impl Root {
    pub fn rebut(&self) -> Ring {
        // println!("strong_count: {}", Weak::strong_count(&self.item));
        if let Some(edge) = self.edge.upgrade() {
            let edge = edge.read().expect(NO_POISON);
            edge.rebut()
        } else {
            Ring::new()
        }
    }
    pub fn react(&self, meta: &Meta) {
        if let Some(edge) = self.edge.upgrade() {
            let edge = edge.read().expect(NO_POISON);
            edge.react(meta);
        }
    }
}

impl Eq for Root {}

impl PartialEq for Root {
    fn eq(&self, other: &Self) -> bool {
        Weak::ptr_eq(&self.edge, &other.edge) && self.meta.id == other.meta.id
    }
}

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

impl Back {
    pub fn rebut(&self) -> Ring {
        // println!("strong_count: {}", Weak::strong_count(&self.item));
        if let Some(node) = self.node.upgrade() {
            let mut node = node.write().expect(NO_POISON);
            node.rebut()
        } else {
            Ring::new()
        }
    }
    pub fn react(&self, meta: &Meta) {
        if let Some(node) = self.node.upgrade() {
            let mut node = node.write().expect(NO_POISON);
            node.react(meta);
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
    pub fn add_root(&mut self, root: Root) {
        self.roots.insert(root);
    }
    pub fn rebut(&self) -> Ring {
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
    pub fn cycle(&mut self, meta: &Meta) {
        let ring = self.rebut();
        self.roots.clear();
        for root in &ring.roots {
            root.react(meta);
        }
    }
    // TODO: make method to remove reactors with dropped edges
}
