use std::hash::Hash;
use std::sync::{Arc, RwLock};

// use serde::Serialize;

use crate::*;

pub type Ace<L> = Edge<node::Ace<L>>;
pub type Deuce<U, L> = Edge<node::Deuce<U, L>>;
pub type Trey<U, T, L> = Edge<node::Trey<U, T, L>>;

/// The bridge between root and stem node.
pub struct Edge<N> {
    pub back: Option<Back>,
    pub node: Arc<RwLock<N>>,
    // pub meta: Meta,
}

impl<N> FromItem for Edge<N>
where
    N: FromItem,
{
    type Item = N::Item;
    fn new(unit: Self::Item) -> Self {
        Self {
            back: None,
            node: Arc::new(RwLock::new(N::new(unit))),
            // meta: Meta::new(),
        }
    }
}

impl<N> Edge<N>
where
    N: 'static + Update + Send + Sync,
{
    fn node_as_back(&self) -> Back {
        let stem = self.node.clone() as Arc<RwLock<dyn Update + Send + Sync>>;
        Back {
            node: Arc::downgrade(&stem),
            // meta: self.meta.clone(),
        }
    }
}

impl<U, L> Produce<L> for Deuce<U, L>
where
    U: Grant<Load = L> + 'static + Send + Sync,
    L: Clone + 'static + Send + Sync,
{
}

impl<N> Grant for Edge<N>
where
    N: Grantor,
{
    type Load = N::Load;
    fn grant(&self) -> Self::Load {
        let mut stem = self.node.write().expect(NO_POISON);
        stem.grantor()
    }
}

impl<U, L> PloyWithBack for Deuce<U, L>
where
    U: Grant<Load = L> + 'static + Send + Sync,
    L: Clone + 'static + Send + Sync,
{
    type Load = L;
    fn ploy_with_back(&self, root: Back) -> Arc<RwLock<dyn Produce<L> + Send + Sync>> {
        Arc::new(RwLock::new(Self {
            back: Some(root),
            node: self.node.clone(),
            // meta: self.meta.clone(),
        }))
    }
}

impl<U, T, L> Convert<T, L> for Trey<U, T, L>
where
    U: Solve<Task = T, Load = L> + Send + Sync + 'static,
    T: Clone + Eq + PartialEq + Hash + Send + Sync + 'static,
    L: Clone + Send + Sync + 'static,
{
}

impl<N> Solve for Edge<N>
where
    N: Solver,
{
    type Task = N::Task;
    type Load = N::Load;
    fn solve(&self, task: Self::Task) -> Self::Load {
        let mut stem = self.node.write().expect(NO_POISON);
        stem.solver(task)
    }
}

impl<U, T, L> PlanWithBack for Trey<U, T, L>
where
    U: Solve<Task = T, Load = L> + Send + Sync + 'static,
    T: Clone + Eq + PartialEq + Hash + Send + Sync + 'static,
    L: Clone + Send + Sync + 'static,
{
    type Task = T;
    type Load = L;
    fn plan_with_back(&self, root: Back) -> Arc<RwLock<dyn Convert<T, L> + Send + Sync>> {
        Arc::new(RwLock::new(Self {
            back: Some(root),
            node: self.node.clone(),
            // meta: self.meta.clone(),
        }))
    }
}

impl<N> ToLoad for Edge<N>
where
    N: ToLoad,
{
    type Load = N::Load;
    fn load(&self) -> Self::Load {
        let stem = self.node.read().expect(NO_POISON);
        stem.load()
    }
}

impl<N> Backed for Edge<N> {
    fn backed(&self, root: &Back) -> Self {
        Self {
            back: Some(root.clone()),
            node: self.node.clone(),
            // meta: self.meta.clone(),
        }
    }
}

impl<N> Writer for Edge<N>
where
    N: Write,
{
    type Item = N::Item;
    fn writer<F: FnOnce(&mut Self::Item)>(&self, write: F) {
        let mut stem = self.node.write().expect(NO_POISON);
        stem.write(write);
    }
}

impl<N> WriterWithPack for Edge<N>
where
    N: 'static + WriteWithBack + Update + Send + Sync,
{
    type Unit = N::Unit;
    fn writer<F: FnOnce(&mut Pack<Self::Unit>)>(&self, write: F) {
        let mut stem = self.node.write().expect(NO_POISON);
        stem.write_with_back(write, &self.node_as_back());
    }
}

impl<N> Reader for Edge<N>
where
    N: Read,
{
    type Item = N::Item;
    fn reader<F: FnOnce(&Self::Item)>(&self, read: F) {
        let stem = self.node.read().expect(NO_POISON);
        read(stem.read());
    }
}

impl<N> AddRoot for Edge<N>
where
    N: AddRoot,
{
    fn add_root(&mut self, root: Root) {
        let mut stem = self.node.write().expect(NO_POISON);
        stem.add_root(root);
    }
}

impl<N> Updater for Edge<N> {}

impl<N> Rebuter for Edge<N> {
    fn rebuter(&self) -> Ring {
        if let Some(root) = &self.back {
            root.rebuter()
        } else {
            Ring::new()
        }
    }
}

impl<N> Reactor for Edge<N> {
    fn reactor(&self, meta: &Meta) {
        if let Some(back) = &self.back {
            back.reactor(meta);
        }
    }
}

// impl<R, St> Serialize for Edge<R, St> {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         self.meta.serialize(serializer)
//     }
// }
