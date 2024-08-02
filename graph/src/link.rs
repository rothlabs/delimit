pub use ace::ToLeaf;

use super::*;
use serde::Serialize;
#[cfg(not(feature = "oneThread"))]
use std::sync::{Arc, RwLock};
#[cfg(feature = "oneThread")]
use std::{cell::RefCell, rc::Rc};

mod ace;
#[cfg(test)]
mod tests;

/// Link to a load. The most simple graph part.
pub type Leaf = Link<edge::Leaf>;

/// Link to a unit that grants a load.
pub type Agent<U> = Link<edge::Agent<U>>;

/// Link that grants a load.
pub type Ploy = Link<Box<dyn Engage>>;

/// Link to an edge that leads to a apex that contains a unit.
/// Units hold links as source of input used to compute output.
pub struct Link<E> {
    #[cfg(not(feature = "oneThread"))]
    edge: Arc<RwLock<E>>,
    #[cfg(feature = "oneThread")]
    edge: Rc<RefCell<E>>,
    meta: Meta,
}

impl<E> ToMeta for Link<E> {
    fn meta(&self) -> Meta {
        self.meta.clone()
    }
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
        let apex = E::new(unit);
        Self {
            meta: apex.meta(),
            #[cfg(not(feature = "oneThread"))]
            edge: Arc::new(RwLock::new(apex)),
            #[cfg(feature = "oneThread")]
            edge: Rc::new(RefCell::new(apex)),
        }
    }
}

impl<E> Maker for Link<E>
where
    E: Maker + ToMeta,
{
    type Unit = E::Unit;
    fn maker<F: FnOnce(&Back) -> Self::Unit>(make: F) -> Self {
        let apex = E::maker(make);
        Self {
            meta: apex.meta(),
            #[cfg(not(feature = "oneThread"))]
            edge: Arc::new(RwLock::new(apex)),
            #[cfg(feature = "oneThread")]
            edge: Rc::new(RefCell::new(apex)),
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

/// TODO: make reader that does not add a root to the apex.
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

impl<E> ReadLoad for Link<E>
where
    E: 'static + ReadLoad + Update + AddRoot,
{
    fn read_load<T, F: FnOnce(load::ResultRef) -> T>(&self, read: F) -> T {
        read_part(&self.edge, |edge| {
            let out = edge.read_load(read);
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

impl<E> Solve for Link<E>
where
    E: 'static + Solve + AddRoot + Update,
{
    fn solve(&self, task: Task) -> solve::Result {
        read_part(&self.edge, |edge| {
            let result = edge.solve(task);
            edge.add_root(self.as_root());
            result
        })
    }
}

impl<E> DoAlter for Link<E>
where
    E: DoAlter,
{
    fn alter(&self, post: Post) -> alter::Result {
        read_part(&self.edge, |edge| edge.alter(post))
    }
}

impl<E> Link<E>
where
    //E: Solve + ToPloy<Load = <E as Solve>::Load>,
    E: ToPloy,
{
    /// Copy the link with unit type erased.  
    pub fn ploy(&self) -> Ploy {
        read_part(&self.edge, |edge| Ploy {
            edge: edge.ploy(),
            meta: self.meta.clone(),
        })
    }
}

impl Backed for Ploy {
    fn backed(&self, back: &Back) -> Self {
        read_part(&self.edge, |edge| Self {
            edge: edge.backed_ploy(back),
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

// impl<E> Link<E>
// where
//     E: Grant + ToPloy<Load = <<E as Grant>::Load as Grant>::Load>,
//     <E as Grant>::Load: Grant,
// {
//     /// Copy the link with unit type erased.
//     pub fn piped_ploy(&self) -> Ploy<<<E as Grant>::Load as Grant>::Load> {
//         read_part(&self.edge, |edge| Ploy {
//             edge: edge.ploy(),
//             meta: self.meta.clone(),
//         })
//     }
// }

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
