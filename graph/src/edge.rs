use std::sync::{Arc, RwLock};
use std::hash::Hash;

// use serde::Serialize;

use crate::*;

pub type Sole<L> = Edge<Root, node::Sole<L>>;
pub type Pair<U, L> = Edge<Root, node::Pair<U, L>>;
pub type Trey<U, T, L> = Edge<Root, node::Trey<U, T, L>>;

/// The bridge between root and stem node.
pub struct Edge<R, S> {
    pub root: Option<R>,
    pub stem: Arc<RwLock<S>>,
    // pub meta: Meta,
}

impl<R, S> FromItem for Edge<R, S>
where
    S: FromItem,
{
    type Item = S::Item;
    fn new(unit: Self::Item) -> Self {
        Self {
            root: None,
            stem: Arc::new(RwLock::new(S::new(unit))),
            // meta: Meta::new(),
        }
    }
}

impl<R, S> Edge<R, S>
where
    S: 'static + Updater + Send + Sync,
{
    fn as_root(&self) -> Root {
        let stem = self.stem.clone() as Arc<RwLock<dyn Updater + Send + Sync>>;
        Root {
            item: Arc::downgrade(&stem),
            // meta: self.meta.clone(),
        }
    }
}

impl<U, L> Formula<L> for Pair<U, L>
where
    U: Grant<Load = L> + 'static + Send + Sync,
    L: Clone + 'static + Send + Sync,
{
}

impl<R, S> Grant for Edge<R, S>
where
    S: Grantor,
{
    type Load = S::Load;
    fn grant(&self) -> Self::Load {
        let mut stem = self.stem.write().expect(NO_POISON);
        stem.grantor()
    }
}

impl<U, L> FormulaWithRoot for Pair<U, L>
where
    U: Grant<Load = L> + 'static + Send + Sync,
    L: Clone + 'static + Send + Sync,
{
    type Load = L;
    fn formula_with_root(
        &self,
        root: Root,
    ) -> Arc<RwLock<dyn Formula<L> + Send + Sync>> {
        Arc::new(RwLock::new(Self {
            root: Some(root),
            stem: self.stem.clone(),
            // meta: self.meta.clone(),
        }))
    }
}

impl<U, T, L> Problem<T, L> for Trey<U, T, L>
where
    U: Solve<Task = T, Load = L> + Send + Sync + 'static,
    T: Clone + Eq + PartialEq + Hash + Send + Sync + 'static,
    L: Clone + Send + Sync + 'static,
{
}

impl<R, S> Solve for Edge<R, S>
where
    S: Solver,
{
    type Task = S::Task;
    type Load = S::Load;
    fn solve(&self, task: Self::Task) -> Self::Load {
        let mut stem = self.stem.write().expect(NO_POISON);
        stem.solver(task)
    }
}

impl<U, T, L> ProblemWithRoot for Trey<U, T, L>
where
    U: Solve<Task = T, Load = L> + Send + Sync + 'static,
    T: Clone + Eq + PartialEq + Hash + Send + Sync + 'static,
    L: Clone + Send + Sync + 'static,
{
    type Task = T;
    type Load = L;
    fn problem_with_root(
        &self,
        root: Root,
    ) -> Arc<RwLock<dyn Problem<T, L> + Send + Sync>> {
        Arc::new(RwLock::new(Self {
            root: Some(root),
            stem: self.stem.clone(),
            // meta: self.meta.clone(),
        }))
    }
}

impl<R, S> ToLoad for Edge<R, S>
where
    S: ToLoad,
{
    type Load = S::Load;
    fn load(&self) -> Self::Load {
        let stem = self.stem.read().expect(NO_POISON);
        stem.load()
    }
}

impl<R, S> WithRoot for Edge<R, S>
where
    R: Clone,
{
    type Root = R;
    fn with_root(&self, root: &R) -> Self {
        Self {
            root: Some(root.clone()),
            stem: self.stem.clone(),
            // meta: self.meta.clone(),
        }
    }
}

impl<R, S> Writer for Edge<R, S>
where
    S: Write,
{
    type Unit = S::Unit;
    fn writer<F: FnOnce(&mut Self::Unit)>(&self, write: F) {
        let mut stem = self.stem.write().expect(NO_POISON);
        stem.write(write);
    }
}

impl<R, S> WriterWithPack for Edge<R, S>
where
    S: 'static + WriteWithRoot + Updater + Send + Sync,
{
    type Unit = S::Unit;
    fn writer<F: FnOnce(&mut Pack<Self::Unit>)>(&self, write: F) {
        let mut stem = self.stem.write().expect(NO_POISON);
        stem.write_with_root(write, &self.as_root());
    }
}

impl<R, S> Reader for Edge<R, S>
where
    S: Read,
{
    type Unit = S::Unit;
    fn reader<F: FnOnce(&Self::Unit)>(&self, read: F) {
        let stem = self.stem.read().expect(NO_POISON);
        read(stem.read());
    }
}

impl<R, S> AddRoot for Edge<R, S>
where
    S: AddRoot,
{
    type Root = S::Root;
    fn add_root(&mut self, root: Self::Root) {
        let mut stem = self.stem.write().expect(NO_POISON);
        stem.add_root(root);
    }
}

impl<R, S> Update for Edge<R, S> where
    R: Rebut<Ring = Ring> + React //  + Send + Sync
                                  //S: Send + Sync
{
}

impl<R, S> Rebut for Edge<R, S>
where
    R: Rebut<Ring = Ring>,
{
    type Ring = R::Ring;
    fn rebut(&self) -> Self::Ring {
        if let Some(root) = &self.root {
            root.rebut()
        } else {
            Ring::new()
        }
    }
}

impl<R, S> React for Edge<R, S>
where
    R: React,
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
