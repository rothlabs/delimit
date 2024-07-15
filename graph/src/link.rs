use std::sync::{Arc, RwLock};

use crate::*;

pub use ace::{IntoAce, ToAce};
use serde::Serialize;

#[cfg(test)]
mod tests;

mod ace;

/// Link to a load.
pub type Ace<L> = Link<edge::Ace<L>>;
pub type Deuce<U, L> = Link<edge::Deuce<U, L>>;
pub type Trey<U, T, L> = Link<edge::Trey<U, T, L>>;
pub type Ploy<L> = Link<dyn Produce<L> + Send + Sync>;
pub type Plan<T, L> = Link<dyn Convert<T, L> + Send + Sync>;

/// Points to one edge which in turn points to one node.
/// Units hold links as source of input used to compute output.
pub struct Link<E: ?Sized> {
    edge: Arc<RwLock<E>>,
    meta: Meta,
}

impl<E> ToLoad for Link<E>
where
    E: ToLoad,
{
    type Load = E::Load;
    fn load(&self) -> Self::Load {
        let edge = self.edge.read().expect(NO_POISON);
        edge.load()
    }
}

impl<E: ?Sized> Clone for Link<E> {
    fn clone(&self) -> Self {
        Self {
            edge: self.edge.clone(),
            meta: self.meta.clone(),
        }
    }
}

impl<E> PartialEq for Link<E> {
    fn eq(&self, other: &Self) -> bool {
        Arc::<RwLock<E>>::ptr_eq(&self.edge, &other.edge) && self.meta == other.meta
    }
}

impl<E> FromItem for Link<E>
where
    E: FromItem,
{
    type Item = E::Item;
    fn new(unit: Self::Item) -> Self {
        Self {
            edge: Arc::new(RwLock::new(E::new(unit))),
            meta: Meta::new(),
        }
    }
}

impl<E> Backed for Link<E>
where
    E: Backed,
{
    type Back = E::Back;
    fn backed(&self, root: &Self::Back) -> Self {
        let edge = self.edge.read().expect(NO_POISON);
        Self {
            edge: Arc::new(RwLock::new(edge.backed(root))),
            meta: self.meta.clone(),
        }
    }
}

impl<E> Link<E>
where
    E: 'static + Updater + Send + Sync,
{
    pub fn as_root(&self) -> Root {
        let edge = self.edge.clone() as Arc<RwLock<dyn Updater + Send + Sync>>;
        Root {
            edge: Arc::downgrade(&edge),
            meta: self.meta.clone(),
        }
    }
}

impl<E> Reader for Link<E>
where
    E: 'static + Reader + Updater + AddRoot<Root = Root> + Send + Sync,
{
    type Item = E::Item;
    fn reader<F: FnOnce(&Self::Item)>(&self, read: F) {
        // TODO: first read and check if it is not added as Back and then write to do so
        let mut edge = self.edge.write().expect(NO_POISON);
        edge.reader(read);
        edge.add_root(self.as_root());
    }
}

impl<E> Writer for Link<E>
where
    E: Writer,
{
    type Item = E::Item;
    fn writer<F: FnOnce(&mut Self::Item)>(&self, write: F) {
        let edge = self.edge.read().expect(NO_POISON);
        edge.writer(write);
    }
}

impl<E> WriterWithPack for Link<E>
where
    E: WriterWithPack,
{
    type Unit = E::Unit;
    fn writer<F: FnOnce(&mut Pack<Self::Unit>)>(&self, write: F) {
        let edge = self.edge.read().expect(NO_POISON);
        edge.writer(write);
    }
}

impl<E: ?Sized> Grant for Link<E>
where
    E: Grant,
{
    type Load = E::Load;
    fn grant(&self) -> Self::Load {
        let edge = self.edge.read().expect(NO_POISON);
        edge.grant()
    }
}

impl<ER, NR, U, L> Link<Edge<ER, Node<NR, work::Deuce<U, L>>>>
where
    ER: 'static + Send + Sync,
    NR: 'static + Send + Sync,
    U: 'static + Send + Sync,
    L: 'static + Send + Sync,
    Edge<ER, Node<NR, work::Deuce<U, L>>>: Produce<L>,
{
    pub fn ploy(&self) -> Ploy<L> {
        let edge = self.edge.clone() as Arc<RwLock<dyn Produce<L> + Send + Sync>>;
        Ploy {
            edge,
            meta: self.meta.clone(),
        }
    }
}

impl<L> Backed for Link<dyn Produce<L> + Send + Sync> {
    type Back = Back;
    fn backed(&self, root: &Back) -> Self {
        let edge = self.edge.read().expect(NO_POISON);
        Self {
            edge: edge.ploy_with_back(root.clone()),
            meta: self.meta.clone(),
        }
    }
}

impl<E: ?Sized> Solve for Link<E>
where
    E: Solve,
{
    type Task = E::Task;
    type Load = E::Load;
    fn solve(&self, task: Self::Task) -> Self::Load {
        let edge = self.edge.read().expect(NO_POISON);
        edge.solve(task)
    }
}

impl<ER, NR, U, T, L> Link<Edge<ER, Node<NR, work::Trey<U, T, L>>>>
where
    ER: 'static + Send + Sync,
    NR: 'static + Send + Sync,
    U: 'static + Send + Sync,
    T: 'static + Send + Sync,
    L: 'static + Send + Sync,
    Edge<ER, Node<NR, work::Trey<U, T, L>>>: Convert<T, L>,
{
    pub fn plan(&self) -> Plan<T, L> {
        let edge = self.edge.clone() as Arc<RwLock<dyn Convert<T, L> + Send + Sync>>;
        Plan {
            edge,
            meta: self.meta.clone(),
        }
    }
}

impl<T, L> Backed for Link<dyn Convert<T, L> + Send + Sync> {
    type Back = Back;
    fn backed(&self, root: &Back) -> Self {
        let edge = self.edge.read().expect(NO_POISON);
        Self {
            edge: edge.plan_with_back(root.clone()),
            meta: self.meta.clone(),
        }
    }
}

impl<E: ?Sized> Serialize for Link<E> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.meta.serialize(serializer)
    }
}

// impl<L> Link<dyn Produce<L> + Send + Sync> {
//     pub fn with_root(&self, root: &Back) -> Self {
//         let edge = self.edge.read().expect(NO_POISON);
//         Self {
//             edge: edge.produce_with_back(root.clone()),
//             meta: self.meta.clone(),
//         }
//     }
// }

// impl<T, L> Link<dyn Convert<T, L> + Send + Sync> {
//     pub fn with_root(&self, root: &Back) -> Self {
//         let edge = self.edge.read().expect(NO_POISON);
//         Self {
//             edge: edge.convert_with_back(root.clone()),
//             meta: self.meta.clone(),
//         }
//     }
// }
