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
pub type Ploy<L> = Link<Box<dyn Produce<L> + Send + Sync>>;
pub type Plan<T, L> = Link<Box<dyn Convert<T, L> + Send + Sync>>;

/// Points to one edge which in turn points to one node.
/// Units hold links as source of input used to compute output.
pub struct Link<E> {
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

impl<E> Clone for Link<E> {
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
    fn backed(&self, root: &Back) -> Self {
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
    E: 'static + Reader + Updater + RootAdder + Send + Sync,
{
    type Item = E::Item;
    fn read<F: FnOnce(&Self::Item)>(&self, read: F) {
        let edge = self.edge.read().expect(NO_POISON);
        edge.read(read);
        edge.add_root(self.as_root());
    }
}

impl<E> Writer for Link<E>
where
    E: Writer,
{
    type Item = E::Item;
    fn write<F: FnOnce(&mut Self::Item)>(&self, write: F) {
        let edge = self.edge.read().expect(NO_POISON);
        edge.write(write);
    }
}

impl<E> WriterWithPack for Link<E>
where
    E: WriterWithPack,
{
    type Unit = E::Unit;
    fn write<F: FnOnce(&mut Pack<Self::Unit>)>(&self, write: F) {
        let edge = self.edge.read().expect(NO_POISON);
        edge.write(write);
    }
}

impl<E> Grant for Link<E>
// E: ?Sized
where
    E: 'static + Grant + RootAdder + Updater + Send + Sync,
{
    type Load = E::Load;
    fn grant(&self) -> Self::Load {
        let edge = self.edge.read().expect(NO_POISON);
        let result = edge.grant();
        edge.add_root(self.as_root());
        result
    }
}

impl<U, L> Link<Edge<Node<work::Deuce<U, L>>>>
where
    Edge<Node<work::Deuce<U, L>>>: ToPloy<Load = L>, //Produce<L>,
{
    pub fn ploy(&self) -> Ploy<L> {
        let edge = self.edge.read().expect(NO_POISON);
        Ploy {
            edge: edge.ploy(),
            meta: self.meta.clone(),
        }
    }
}

impl<L> Backed for Link<Box<dyn Produce<L> + Send + Sync>> {
    fn backed(&self, root: &Back) -> Self {
        let edge = self.edge.read().expect(NO_POISON);
        Self {
            edge: edge.ploy_with_back(root.clone()),
            meta: self.meta.clone(),
        }
    }
}

impl<E> Solve for Link<E>
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

impl<U, T, L> Link<Edge<Node<work::Trey<U, T, L>>>>
where
    Edge<Node<work::Trey<U, T, L>>>: ToPlan<Task = T, Load = L>, //Convert<T, L>,
{
    pub fn plan(&self) -> Plan<T, L> {
        let edge = self.edge.read().expect(NO_POISON);
        Plan {
            edge: edge.plan(),
            meta: self.meta.clone(),
        }
    }
}

impl<T, L> Backed for Link<Box<dyn Convert<T, L> + Send + Sync>> {
    fn backed(&self, root: &Back) -> Self {
        let edge = self.edge.read().expect(NO_POISON);
        Self {
            edge: edge.plan_with_back(root.clone()),
            meta: self.meta.clone(),
        }
    }
}

impl<E> Serialize for Link<E> {
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
