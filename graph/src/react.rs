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

pub trait WithRoot {
    type Root;
    fn with_root(&self, root: &Self::Root) -> Self;
}

pub trait FormulaWithRoot {
    type Load;
    fn formula_with_root(
        &self,
        root: Back,
    ) -> Arc<RwLock<dyn Formula<Self::Load> + Send + Sync>>;
}

pub trait ProblemWithRoot {
    type Task;
    type Load;
    fn problem_with_root(
        &self,
        root: Back,
    ) -> Arc<RwLock<dyn Problem<Self::Task, Self::Load> + Send + Sync>>;
}

pub trait Cycle {
    fn cycle(&mut self);
}

pub trait Update: Rebut<Ring = Ring> + React {} 

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
        if let Some(item) = self.edge.upgrade() {
            let item = item.read().expect(NO_POISON);
            item.rebut()
        } else {
            Ring::new()
        }
    }
}

impl React for Root {
    fn react(&self) {
        if let Some(item) = self.edge.upgrade() {
            let item = item.read().expect(NO_POISON);
            item.react();
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

/// Weakly point to a root node as Updater.
#[derive(Clone)]
pub struct Back {
    pub item: Weak<RwLock<dyn Updater + Send + Sync + 'static>>,
    // pub meta: Meta,
}

impl Rebut for Back {
    type Ring = Ring;
    fn rebut(&self) -> Self::Ring {
        // println!("strong_count: {}", Weak::strong_count(&self.item));
        if let Some(item) = self.item.upgrade() {
            let mut item = item.write().expect(NO_POISON);
            item.rebuter()
        } else {
            Ring::new()
        }
    }
}

impl React for Back {
    fn react(&self) {
        if let Some(item) = self.item.upgrade() {
            let mut item = item.write().expect(NO_POISON);
            item.reactor();
        }
    }
}

/// Points to many root edges, each pointing to a root node.
#[derive(Default)]
pub struct Ring {
    backs: HashSet<Root>,
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
        self.backs.clear();
        for back in &ring.backs {
            back.react();
        }
    }
}

impl Rebut for Ring {
    type Ring = Self;
    fn rebut(&self) -> Self::Ring {
        let mut ring = Ring::new();
        for back in &self.backs {
            let root_ring = back.rebut();
            if root_ring.backs.is_empty() {
                ring.backs.insert(back.clone());
            } else {
                ring.backs.extend(root_ring.backs);
            }
        }
        ring
    }
}

impl AddRoot for Ring {
    type Root = Root;
    fn add_root(&mut self, root: Self::Root) {
        self.backs.insert(root);
    }
}