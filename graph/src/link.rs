use std::sync::{Arc, RwLock};

use crate::*;

pub use ace::{IntoAce, ToAce};
use serde::Serialize;

#[cfg(test)]
mod tests;

mod ace;

/// Link to a load.
pub type Ace<L> = Link<edge::Ace<L>>;

/// Link to a unit that grants a load.
pub type Deuce<U, L> = Link<edge::Deuce<U, L>>;

/// Link to a unit that solves a task with resulting load.
pub type Trey<U, T, L> = Link<edge::Trey<U, T, L>>;

/// Link that grants a load.
pub type Ploy<L> = Link<Box<dyn Produce<L> + Send + Sync>>;

/// Link that solves a task with resulting load.
pub type Plan<T, L> = Link<Box<dyn Convert<T, L> + Send + Sync>>;

/// Link to an edge that leads to a node that contains a unit.
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
    fn backed(&self, back: &Back) -> Self {
        let edge = self.edge.read().expect(NO_POISON);
        Self {
            edge: Arc::new(RwLock::new(edge.backed(back))),
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
    fn read<T, F: FnOnce(&Self::Item) -> T>(&self, read: F) -> T {
        let edge = self.edge.read().expect(NO_POISON);
        let out = edge.read(read);
        edge.add_root(self.as_root());
        out
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

impl<E> Solve for Link<E>
where
    E: 'static + Solve + RootAdder + Updater + Send + Sync,
{
    type Task = E::Task;
    type Load = E::Load;
    fn solve(&self, task: Self::Task) -> Self::Load {
        let edge = self.edge.read().expect(NO_POISON);
        let result = edge.solve(task);
        edge.add_root(self.as_root());
        result
    }
}

impl<E> Link<E>
where
    E: Grant + ToPloy<Load = <E as Grant>::Load>,
{
    pub fn ploy(&self) -> Ploy<<E as Grant>::Load> {
        let edge = self.edge.read().expect(NO_POISON);
        Ploy {
            edge: edge.ploy(),
            meta: self.meta.clone(),
        }
    }
}

impl<L> Backed for Link<Box<dyn Produce<L> + Send + Sync>> {
    fn backed(&self, back: &Back) -> Self {
        let edge = self.edge.read().expect(NO_POISON);
        Self {
            edge: edge.backed_ploy(back),
            meta: self.meta.clone(),
        }
    }
}

impl<E> Link<E>
where
    E: Solve + ToPlan<Task = <E as Solve>::Task, Load = <E as Solve>::Load>,
{
    pub fn plan(&self) -> Plan<<E as Solve>::Task, <E as Solve>::Load> {
        let edge = self.edge.read().expect(NO_POISON);
        Plan {
            edge: edge.plan(),
            meta: self.meta.clone(),
        }
    }
}

impl<T, L> Backed for Link<Box<dyn Convert<T, L> + Send + Sync>> {
    fn backed(&self, back: &Back) -> Self {
        let edge = self.edge.read().expect(NO_POISON);
        Self {
            edge: edge.backed_plan(back),
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

// impl<U, L> Link<Edge<Node<work::Deuce<U, L>>>>
// where
//     Edge<Node<work::Deuce<U, L>>>: ToPloy<Load = L>,
// {
//     pub fn ploy(&self) -> Ploy<L> {
//         let edge = self.edge.read().expect(NO_POISON);
//         Ploy {
//             edge: edge.ploy(),
//             meta: self.meta.clone(),
//         }
//     }
// }

// impl<U, T, L> Link<Edge<Node<work::Trey<U, T, L>>>>
// where
//     Edge<Node<work::Trey<U, T, L>>>: ToPlan<Task = T, Load = L>, //Convert<T, L>,
// {
//     pub fn plan(&self) -> Plan<T, L> {
//         let edge = self.edge.read().expect(NO_POISON);
//         Plan {
//             edge: edge.plan(),
//             meta: self.meta.clone(),
//         }
//     }
// }
