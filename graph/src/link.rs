pub use ace::{IntoAce, ToAce};

use super::*;
use serde::Serialize;
#[cfg(not(feature = "oneThread"))]
use std::sync::{Arc, RwLock};
#[cfg(feature = "oneThread")]
use std::{cell::RefCell, rc::Rc};

mod ace;
#[cfg(test)]
mod tests;

///////////////////////////////////////////////
// /// impl Read and ToLoad
// pub enum Asset<L> {
//     Link(Link<Box<dyn Produce<Ace<L>>>>),
//     Bare(L)
// }
/// ///////////////////////////////////////////

/// Link to a load. The most simple graph part.
pub type Ace<L> = Link<edge::Ace<L>>;

/// Link to a unit that grants a load.
pub type Deuce<U> = Link<edge::Deuce<U>>;

/// Link to a unit that solves a task with resulting load.
pub type Trey<U, T, L> = Link<edge::Trey<U, T, L>>;

/// Link to a unit that grants a load.
/// Unlike Deuce, the agent will act upon some external system.
pub type Agent<U> = Link<edge::Agent<U>>;

/// Link to a unit that solves a task and could act upon externals.
pub type Envoy<U> = Link<edge::Envoy<U>>;

/// Link that grants a load.
pub type Ploy<L> = Link<Box<dyn Produce<L>>>;

/// Link that grants an Ace.
// pub type OldAsset<L> = Ploy<Ace<L>>;

/// Link that solves a task with resulting load.
pub type Plan<T, L> = Link<Box<dyn Convert<T, L>>>;

/// Link that grants a load of a intermediate. The unit should
/// be a link. The pipe will react to both the unit and intermediate.
/// The intermediate is a sub graph of the super graph unit.
pub type Pipe<U> = Link<edge::Pipe<U>>;

/// Link to an edge that leads to a node that contains a unit.
/// Units hold links as source of input used to compute output.
pub struct Link<E> {
    #[cfg(not(feature = "oneThread"))]
    edge: Arc<RwLock<E>>,
    #[cfg(feature = "oneThread")]
    edge: Rc<RefCell<E>>,
    meta: Meta,
}

impl<E> ToLoad for Link<E>
where
    E: ToLoad,
{
    type Load = E::Load;
    fn load(&self) -> Self::Load {
        read_part(&self.edge, |edge| edge.load())
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
    #[cfg(not(feature = "oneThread"))]
    fn eq(&self, other: &Self) -> bool {
        Arc::<RwLock<E>>::ptr_eq(&self.edge, &other.edge) && self.meta == other.meta
    }
    #[cfg(feature = "oneThread")]
    fn eq(&self, other: &Self) -> bool {
        Rc::<RefCell<E>>::ptr_eq(&self.edge, &other.edge) && self.meta == other.meta
    }
}

impl<E> FromItem for Link<E>
where
    E: FromItem + ToMeta,
{
    type Item = E::Item;
    fn new(unit: Self::Item) -> Self {
        let node = E::new(unit);
        Self {
            meta: node.meta(),
            #[cfg(not(feature = "oneThread"))]
            edge: Arc::new(RwLock::new(node)),
            #[cfg(feature = "oneThread")]
            edge: Rc::new(RefCell::new(node)),
        }
    }
}

impl<E> Make for Link<E>
where
    E: Make + ToMeta,
{
    type Unit = E::Unit;
    fn make<F: FnOnce(&Back) -> Self::Unit>(make: F) -> Self {
        let node = E::make(make);
        Self {
            meta: node.meta(),
            #[cfg(not(feature = "oneThread"))]
            edge: Arc::new(RwLock::new(node)),
            #[cfg(feature = "oneThread")]
            edge: Rc::new(RefCell::new(node)),
        }
    }
}

/// TODO: make method to make new link with cloned edge without Back!
impl<E> Backed for Link<E>
where
    E: Backed,
{
    #[cfg(not(feature = "oneThread"))]
    fn backed(&self, back: &Back) -> Self {
        let edge = self.edge.read().expect(NO_POISON);
        Self {
            edge: Arc::new(RwLock::new(edge.backed(back))),
            meta: self.meta.clone(),
        }
    }
    #[cfg(feature = "oneThread")]
    fn backed(&self, back: &Back) -> Self {
        let edge = self.edge.borrow();
        Self {
            edge: Rc::new(RefCell::new(edge.backed(back))),
            meta: self.meta.clone(),
        }
    }
}

impl<E> Link<E>
where
    E: 'static + Update,
{
    #[cfg(not(feature = "oneThread"))]
    pub fn as_root(&self) -> Root {
        let edge = self.edge.clone() as Arc<RwLock<dyn Update>>;
        Root {
            edge: Arc::downgrade(&edge),
            meta: self.meta.clone(),
        }
    }
    #[cfg(feature = "oneThread")]
    pub fn as_root(&self) -> Root {
        let edge = self.edge.clone() as Rc<RefCell<dyn Update>>;
        Root {
            edge: Rc::downgrade(&edge),
            meta: self.meta.clone(),
        }
    }
}

/// TODO: make reader that does not add a root to the node.
/// This will allow readers to inspect without rebuting in the future.
impl<E> Read for Link<E>
where
    E: 'static + Read + Update + AddRoot,
{
    type Item = E::Item;
    fn read<T, F: FnOnce(&Self::Item) -> T>(&self, read: F) -> T {
        read_part(&self.edge, |edge| {
            let out = edge.read(read);
            edge.add_root(self.as_root());
            out
        })
    }
}

impl<E> WriteLoad for Link<E>
where
    E: WriteLoad,
{
    type Item = E::Item;
    fn write<T, F: FnOnce(&mut Self::Item) -> T>(&self, write: F) -> write::Result<T> {
        read_part(&self.edge, |edge| edge.write(write))
    }
}

impl<E> WriteUnit for Link<E>
where
    E: WriteUnit,
{
    type Unit = E::Unit;
    fn write<T, F: FnOnce(&mut Pack<Self::Unit>) -> T>(&self, write: F) -> write::Result<T> {
        read_part(&self.edge, |edge| edge.write(write))
    }
}

impl<E> Grant for Link<E>
where
    E: 'static + Grant + AddRoot + Update,
{
    type Load = E::Load;
    fn grant(&self) -> Self::Load {
        read_part(&self.edge, |edge| {
            let result = edge.grant();
            edge.add_root(self.as_root());
            result
        })
    }
}

impl<E> Act for Link<E>
where
    E: 'static + Act + AddRoot + Update,
{
    type Load = E::Load;
    fn act(&self) -> Self::Load {
        read_part(&self.edge, |edge| {
            let result = edge.act();
            edge.add_root(self.as_root());
            result
        })
    }
}

impl<E> Solve for Link<E>
where
    E: 'static + Solve + AddRoot + Update,
{
    type Task = E::Task;
    type Load = E::Load;
    fn solve(&self, task: Self::Task) -> Self::Load {
        read_part(&self.edge, |edge| {
            let result = edge.solve(task);
            edge.add_root(self.as_root());
            result
        })
    }
}

impl<E> Serve for Link<E>
where
    E: 'static + Serve + AddRoot + Update,
{
    type Task = E::Task;
    type Load = E::Load;
    fn serve(&self, task: Self::Task) -> Self::Load {
        read_part(&self.edge, |edge| {
            let result = edge.serve(task);
            edge.add_root(self.as_root());
            result
        })
    }
}

impl<E> Link<E>
where
    E: Grant + ToPloy<Load = <E as Grant>::Load>,
{
    /// Copy the link with unit type erased.  
    pub fn ploy(&self) -> Ploy<<E as Grant>::Load> {
        read_part(&self.edge, |edge| Ploy {
            edge: edge.ploy(),
            meta: self.meta.clone(),
        })
    }
}

impl<E> Link<E>
where
    E: Grant + ToPloy<Load = <<E as Grant>::Load as Grant>::Load>,
    <E as Grant>::Load: Grant
{
    /// Copy the link with unit type erased.  
    pub fn piped_ploy(&self) -> Ploy<<<E as Grant>::Load as Grant>::Load> {
        read_part(&self.edge, |edge| Ploy {
            edge: edge.ploy(),
            meta: self.meta.clone(),
        })
    }
}

impl<L> Backed for Link<Box<dyn Produce<L>>> {
    fn backed(&self, back: &Back) -> Self {
        read_part(&self.edge, |edge| Self {
            edge: edge.backed_ploy(back),
            meta: self.meta.clone(),
        })
    }
}

impl<E> Link<E>
where
    E: Solve + ToPlan<Task = <E as Solve>::Task, Load = <E as Solve>::Load>,
{
    pub fn plan(&self) -> Plan<<E as Solve>::Task, <E as Solve>::Load> {
        read_part(&self.edge, |edge| Plan {
            edge: edge.plan(),
            meta: self.meta.clone(),
        })
    }
}

impl<T, L> Backed for Link<Box<dyn Convert<T, L>>> {
    fn backed(&self, back: &Back) -> Self {
        read_part(&self.edge, |edge| Self {
            edge: edge.backed_plan(back),
            meta: self.meta.clone(),
        })
    }
}

impl<T> Backed for Vec<T>
where
    T: Backed,
{
    fn backed(&self, back: &Back) -> Self {
        self.iter().map(|link| link.backed(back)).collect()
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


// impl<'a> From<&'a str> for OldAsset<String> {
//     fn from(load: &'a str) -> Self {
//         unit::Asset::link(load.into()).ploy()
//     }
// }

// impl<T> From<T> for OldAsset<T>
// where
//     T: 'static + Clone + SendSync,
// {
//     fn from(load: T) -> Self {
//         unit::Asset::link(load).ploy()
//     }
// }