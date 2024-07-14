use std::hash::Hash;
use std::sync::{Arc, RwLock};

// use serde::Serialize;

use crate::*;

pub type Ace<L> = Edge<Back, node::Ace<L>>;
pub type Deuce<U, L> = Edge<Back, node::Deuce<U, L>>;
pub type Trey<U, T, L> = Edge<Back, node::Trey<U, T, L>>;

/// The bridge between root and stem node.
pub struct Edge<B, N> {
    pub root: Option<B>,
    pub stem: Arc<RwLock<N>>,
    // pub meta: Meta,
}

impl<B, N> FromItem for Edge<B, N>
where
    N: FromItem,
{
    type Item = N::Item;
    fn new(unit: Self::Item) -> Self {
        Self {
            root: None,
            stem: Arc::new(RwLock::new(N::new(unit))),
            // meta: Meta::new(),
        }
    }
}

impl<B, N> Edge<B, N>
where
    N: 'static + Updater + Send + Sync,
{
    fn as_root(&self) -> Back {
        let stem = self.stem.clone() as Arc<RwLock<dyn Updater + Send + Sync>>;
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
        let mut stem = self.stem.write().expect(NO_POISON);
        stem.grantor()
    }
}

impl<U, L> ProduceWithBack for Deuce<U, L>
where
    U: Grant<Load = L> + 'static + Send + Sync,
    L: Clone + 'static + Send + Sync,
{
    type Load = L;
    fn produce_with_back(&self, root: Back) -> Arc<RwLock<dyn Produce<L> + Send + Sync>> {
        Arc::new(RwLock::new(Self {
            root: Some(root),
            stem: self.stem.clone(),
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
        let mut stem = self.stem.write().expect(NO_POISON);
        stem.solver(task)
    }
}

impl<U, T, L> ConvertWithBack for Trey<U, T, L>
where
    U: Solve<Task = T, Load = L> + Send + Sync + 'static,
    T: Clone + Eq + PartialEq + Hash + Send + Sync + 'static,
    L: Clone + Send + Sync + 'static,
{
    type Task = T;
    type Load = L;
    fn convert_with_back(&self, root: Back) -> Arc<RwLock<dyn Convert<T, L> + Send + Sync>> {
        Arc::new(RwLock::new(Self {
            root: Some(root),
            stem: self.stem.clone(),
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
        let stem = self.stem.read().expect(NO_POISON);
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
            root: Some(root.clone()),
            stem: self.stem.clone(),
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
        let mut stem = self.stem.write().expect(NO_POISON);
        stem.write(write);
    }
}

impl<B, N> WriterWithPack for Edge<B, N>
where
    N: 'static + WriteWithBack + Updater + Send + Sync,
{
    type Unit = N::Unit;
    fn writer<F: FnOnce(&mut Pack<Self::Unit>)>(&self, write: F) {
        let mut stem = self.stem.write().expect(NO_POISON);
        stem.write_with_back(write, &self.as_root());
    }
}

impl<B, N> Reader for Edge<B, N>
where
    N: Read,
{
    type Item = N::Item;
    fn reader<F: FnOnce(&Self::Item)>(&self, read: F) {
        let stem = self.stem.read().expect(NO_POISON);
        read(stem.read());
    }
}

impl<B, N> AddRoot for Edge<B, N>
where
    N: AddRoot,
{
    type Root = N::Root;
    fn add_root(&mut self, root: Self::Root) {
        let mut stem = self.stem.write().expect(NO_POISON);
        stem.add_root(root);
    }
}

impl<B, N> Update for Edge<B, N> where
    B: Rebut<Ring = Ring> + React //  + Send + Sync
                                  //S: Send + Sync
{
}

impl<B, N> Rebut for Edge<B, N>
where
    B: Rebut<Ring = Ring>,
{
    type Ring = B::Ring;
    fn rebut(&self) -> Self::Ring {
        if let Some(root) = &self.root {
            root.rebut()
        } else {
            Ring::new()
        }
    }
}

impl<B, N> React for Edge<B, N>
where
    B: React,
{
    fn react(&self) {
        if let Some(root) = &self.root {
            root.react();
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
