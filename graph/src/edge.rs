use std::hash::Hash;
use std::sync::{Arc, RwLock};

// use serde::Serialize;

use crate::*;

pub type Ace<L> = Edge<Back, node::Ace<L>>;
pub type Deuce<U, L> = Edge<Back, node::Deuce<U, L>>;
pub type Trey<U, T, L> = Edge<Back, node::Trey<U, T, L>>;

/// The bridge between root and stem node.
pub struct Edge<B, N> {
    pub back: Option<B>,
    pub node: Arc<RwLock<N>>,
    // pub meta: Meta,
}

impl<B, N> FromItem for Edge<B, N>
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

impl<B, N> Edge<B, N>
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

impl<B, N> Grant for Edge<B, N>
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

impl<B, N> Solve for Edge<B, N>
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

impl<B, N> ToLoad for Edge<B, N>
where
    N: ToLoad,
{
    type Load = N::Load;
    fn load(&self) -> Self::Load {
        let stem = self.node.read().expect(NO_POISON);
        stem.load()
    }
}

impl<B, N> Backed for Edge<B, N>
where
    B: Clone,
{
    type Back = B;
    fn backed(&self, root: &B) -> Self {
        Self {
            back: Some(root.clone()),
            node: self.node.clone(),
            // meta: self.meta.clone(),
        }
    }
}

impl<B, N> Writer for Edge<B, N>
where
    N: Write,
{
    type Item = N::Item;
    fn writer<F: FnOnce(&mut Self::Item)>(&self, write: F) {
        let mut stem = self.node.write().expect(NO_POISON);
        stem.write(write);
    }
}

impl<B, N> WriterWithPack for Edge<B, N>
where
    N: 'static + WriteWithBack + Update + Send + Sync,
{
    type Unit = N::Unit;
    fn writer<F: FnOnce(&mut Pack<Self::Unit>)>(&self, write: F) {
        let mut stem = self.node.write().expect(NO_POISON);
        stem.write_with_back(write, &self.node_as_back());
    }
}

impl<B, N> Reader for Edge<B, N>
where
    N: Read,
{
    type Item = N::Item;
    fn reader<F: FnOnce(&Self::Item)>(&self, read: F) {
        let stem = self.node.read().expect(NO_POISON);
        read(stem.read());
    }
}

impl<B, N> AddRoot for Edge<B, N>
where
    N: AddRoot,
{
    type Root = N::Root;
    fn add_root(&mut self, root: Self::Root) {
        let mut stem = self.node.write().expect(NO_POISON);
        stem.add_root(root);
    }
}

impl<B, N> Updater for Edge<B, N> where B: Rebuter<Ring = Ring> + Reactor {}

impl<B, N> Rebuter for Edge<B, N>
where
    B: Rebuter<Ring = Ring>,
{
    type Ring = B::Ring;
    fn rebut(&self) -> Self::Ring {
        if let Some(root) = &self.back {
            root.rebut()
        } else {
            Ring::new()
        }
    }
}

impl<B, N> Reactor for Edge<B, N>
where
    B: Reactor,
{
    fn reactor(&self) {
        if let Some(back) = &self.back {
            back.reactor();
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
