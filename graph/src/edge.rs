use std::hash::Hash;
use std::sync::{Arc, RwLock};

// use serde::Serialize;

use crate::*;

pub type Ace<L> = Edge<node::Ace<L>>;
pub type Deuce<U, L> = Edge<node::Deuce<U, L>>;
pub type Trey<U, T, L> = Edge<node::Trey<U, T, L>>;

/// The forward bridge between nodes.
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
        let node = self.node.clone() as Arc<RwLock<dyn Update + Send + Sync>>;
        Back {
            node: Arc::downgrade(&node),
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
        let mut node = self.node.write().expect(NO_POISON);
        node.grantor()
    }
}

impl<U, L> ToPloy for Deuce<U, L>
where
    U: Grant<Load = L> + 'static + Send + Sync,
    L: Clone + 'static + Send + Sync,
{
    type Load = L;
    fn ploy(&self) -> Arc<RwLock<Box<dyn Produce<Self::Load> + Send + Sync>>> {
        Arc::new(RwLock::new(Box::new(Self {
            back: self.back.clone(),
            node: self.node.clone(),
            // meta: self.meta.clone(),
        })))
    }
}

impl<U, L> BackedPloy for Deuce<U, L>
where
    U: Grant<Load = L> + 'static + Send + Sync,
    L: Clone + 'static + Send + Sync,
{
    type Load = L;
    fn backed_ploy(&self, back: &Back) -> Arc<RwLock<Box<dyn Produce<L> + Send + Sync>>> {
        Arc::new(RwLock::new(Box::new(Self {
            back: Some(back.clone()),
            node: self.node.clone(),
            // meta: self.meta.clone(),
        })))
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
        let mut node = self.node.write().expect(NO_POISON);
        node.solver(task)
    }
}

impl<U, T, L> ToPlan for Trey<U, T, L>
where
    U: Solve<Task = T, Load = L> + Send + Sync + 'static,
    T: Clone + Eq + PartialEq + Hash + Send + Sync + 'static,
    L: Clone + Send + Sync + 'static,
{
    type Task = T;
    type Load = L;
    fn plan(&self) -> Arc<RwLock<Box<dyn Convert<Self::Task, Self::Load> + Send + Sync>>> {
        Arc::new(RwLock::new(Box::new(Self {
            back: self.back.clone(),
            node: self.node.clone(),
            // meta: self.meta.clone(),
        })))
    }
}

impl<U, T, L> BackedPlan for Trey<U, T, L>
where
    U: Solve<Task = T, Load = L> + Send + Sync + 'static,
    T: Clone + Eq + PartialEq + Hash + Send + Sync + 'static,
    L: Clone + Send + Sync + 'static,
{
    type Task = T;
    type Load = L;
    fn backed_plan(&self, root: Back) -> Arc<RwLock<Box<dyn Convert<T, L> + Send + Sync>>> {
        Arc::new(RwLock::new(Box::new(Self {
            back: Some(root),
            node: self.node.clone(),
            // meta: self.meta.clone(),
        })))
    }
}

impl<N> ToLoad for Edge<N>
where
    N: ToLoad,
{
    type Load = N::Load;
    fn load(&self) -> Self::Load {
        let node = self.node.read().expect(NO_POISON);
        node.load()
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
    fn write<F: FnOnce(&mut Self::Item)>(&self, write: F) {
        let mut node = self.node.write().expect(NO_POISON);
        node.write(write);
    }
}

impl<N> WriterWithPack for Edge<N>
where
    N: 'static + WriteWithBack + Update + Send + Sync,
{
    type Unit = N::Unit;
    fn write<F: FnOnce(&mut Pack<Self::Unit>)>(&self, write: F) {
        let mut node = self.node.write().expect(NO_POISON);
        node.write_with_back(write, &self.node_as_back());
    }
}

impl<N> Reader for Edge<N>
where
    N: Read,
{
    type Item = N::Item;
    fn read<F: FnOnce(&Self::Item)>(&self, read: F) {
        let node = self.node.read().expect(NO_POISON);
        read(node.read());
    }
}

impl<N> RootAdder for Edge<N>
where
    N: AddRoot,
{
    fn add_root(&self, root: Root) {
        let mut node = self.node.write().expect(NO_POISON);
        node.add_root(root);
    }
}

impl<N> Updater for Edge<N> {}

impl<N> Rebuter for Edge<N> {
    fn rebut(&self) -> Ring {
        if let Some(back) = &self.back {
            back.rebut()
        } else {
            Ring::new()
        }
    }
}

impl<N> Reactor for Edge<N> {
    fn react(&self, meta: &Meta) {
        if let Some(back) = &self.back {
            back.react(meta);
        }
    }
}

impl<L> Grant for Box<dyn Produce<L> + Send + Sync> {
    type Load = L;
    fn grant(&self) -> Self::Load {
        self.as_ref().grant()
    }
}

impl<L> RootAdder for Box<dyn Produce<L> + Send + Sync> {
    fn add_root(&self, root: Root) {
        self.as_ref().add_root(root)
    }
}

impl<L> Updater for Box<dyn Produce<L> + Send + Sync> {}

impl<L> Rebuter for Box<dyn Produce<L> + Send + Sync> {
    fn rebut(&self) -> Ring {
        self.as_ref().rebut()
    }
}

impl<L> Reactor for Box<dyn Produce<L> + Send + Sync> {
    fn react(&self, meta: &Meta) {
        self.as_ref().react(meta)
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
