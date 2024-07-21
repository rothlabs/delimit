pub use ace::{IntoAce, ToAce};

use super::*;
use serde::Serialize;
#[cfg(not(feature="oneThread"))]
use std::sync::{Arc, RwLock};
#[cfg(feature="oneThread")]
use std::{cell::RefCell, rc::Rc};

mod ace;
#[cfg(test)]
mod tests;

/// Link to a load.
pub type Ace<L> = Link<edge::Ace<L>>;

/// Link to a unit that grants a load.
pub type Deuce<U> = Link<edge::Deuce<U>>;

/// Link to a unit that solves a task with resulting load.
pub type Trey<U, T, L> = Link<edge::Trey<U, T, L>>;

/// Link to a unit that grants a load.
/// Unlike Deuce, the agent will act upon some external system.
pub type Agent<U> = Link<edge::Agent<U>>;

/// Link that grants a load.
pub type Ploy<L> = Link<Box<dyn Produce<L>>>; //  + Send + Sync

/// Link that solves a task with resulting load.
pub type Plan<T, L> = Link<Box<dyn Convert<T, L>>>; //  + Send + Sync

/// Link that grants a load of a intermediate. The unit should
/// be a link. The pipe will react to both the unit and intermediate.
/// The intermediate is a sub graph of the super graph unit.
pub type Pipe<U> = Link<edge::Pipe<U>>;

/// Link to an edge that leads to a node that contains a unit.
/// Units hold links as source of input used to compute output.
pub struct Link<E> {
    #[cfg(not(feature="oneThread"))]
    edge: Arc<RwLock<E>>,
    #[cfg(feature="oneThread")] 
    edge: Rc<RefCell<E>>,
    meta: Meta,
}

impl<E> ToLoad for Link<E>
where
    E: ToLoad,
{
    type Load = E::Load;
    fn load(&self) -> Self::Load {
        // #[cfg(not(feature="oneThread"))]
        // let edge = self.edge.read().expect(NO_POISON);
        // #[cfg(feature="oneThread")] 
        // let edge = self.edge.borrow();
        //edge.load()
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
    fn eq(&self, other: &Self) -> bool {
        #[cfg(not(feature="oneThread"))]
        let out = Arc::<RwLock<E>>::ptr_eq(&self.edge, &other.edge) && self.meta == other.meta;
        #[cfg(feature="oneThread")] 
        let out = Rc::<RefCell<E>>::ptr_eq(&self.edge, &other.edge) && self.meta == other.meta;
        out
    }
}

impl<E> FromItem for Link<E>
where
    E: FromItem,
{
    type Item = E::Item;
    fn new(unit: Self::Item) -> Self {
        Self {
            #[cfg(not(feature="oneThread"))]
            edge: Arc::new(RwLock::new(E::new(unit))),
            #[cfg(feature="oneThread")] 
            edge: Rc::new(RefCell::new(E::new(unit))),
            meta: Meta::new(),
        }
    }
}

impl<E> Make for Link<E>
where
    E: Make,
{
    type Unit = E::Unit;
    fn make<F: FnOnce(&Back) -> Self::Unit>(make: F) -> Self {
        Self {
            #[cfg(not(feature="oneThread"))]
            edge: Arc::new(RwLock::new(E::make(make))),
            #[cfg(feature="oneThread")] 
            edge: Rc::new(RefCell::new(E::make(make))),
            meta: Meta::new(),
        }
    }
}

/// TODO: make method to make new link with cloned edge without Back!
impl<E> Backed for Link<E>
where
    E: Backed,
{
    fn backed(&self, back: &Back) -> Self {
        #[cfg(not(feature="oneThread"))]
        let edge = self.edge.read().expect(NO_POISON);
        #[cfg(feature="oneThread")] 
        let edge = self.edge.borrow();
        Self {
            #[cfg(not(feature="oneThread"))]
            edge: Arc::new(RwLock::new(edge.backed(back))),
            #[cfg(feature="oneThread")] 
            edge: Rc::new(RefCell::new(edge.backed(back))),
            meta: self.meta.clone(),
        }
    }
}

impl<E> Link<E>
where
    E: 'static + Update,
{
    pub fn as_root(&self) -> Root {
        #[cfg(not(feature="oneThread"))]
        let edge = self.edge.clone() as Arc<RwLock<dyn Update>>; //  + Send + Sync
        #[cfg(feature="oneThread")] 
        let edge = self.edge.clone() as Rc<RefCell<dyn Update>>;
        Root {
            #[cfg(not(feature="oneThread"))]
            edge: Arc::downgrade(&edge),
            #[cfg(feature="oneThread")] 
            edge: Rc::downgrade(&edge),
            meta: self.meta.clone(),
        }
    }
}

/// TODO: make reader that does not add a root to the node.
/// This will allow readers to inspect without rebuting in the future.
impl<E> Read for Link<E>
where
    E: 'static + Read + Update + AddRoot, // + Send + Sync,
{
    type Item = E::Item;
    fn read<T, F: FnOnce(&Self::Item) -> T>(&self, read: F) -> T {
        read_part(&self.edge, |edge| {
            let out = edge.read(read);
            edge.add_root(self.as_root());
            out
        })
        // let edge = self.edge.read().expect(NO_POISON);
        // let out = edge.read(read);
        // edge.add_root(self.as_root());
        // out
    }
}

// target_family = "wasm"

impl<E> Write for Link<E>
where
    E: Write,
{
    type Item = E::Item;
    fn write<F: FnOnce(&mut Self::Item)>(&self, write: F) {
        read_part(&self.edge, |edge| edge.write(write));
        // let edge = self.edge.read().expect(NO_POISON);
        // edge.write(write);
    }
}

impl<E> WriteWithPack for Link<E>
where
    E: WriteWithPack,
{
    type Unit = E::Unit;
    fn write<F: FnOnce(&mut Pack<Self::Unit>)>(&self, write: F) {
        read_part(&self.edge, |edge| edge.write(write));
        // let edge = self.edge.read().expect(NO_POISON);
        // edge.write(write);
    }
}

impl<E> Grant for Link<E>
where
    E: 'static + Grant + AddRoot + Update, // + Send + Sync,
{
    type Load = E::Load;
    fn grant(&self) -> Self::Load {
        read_part(&self.edge, |edge| {
            let result = edge.grant();
            edge.add_root(self.as_root());
            result
        })
        // let edge = self.edge.read().expect(NO_POISON);
        // let result = edge.grant();
        // edge.add_root(self.as_root());
        // result
    }
}

impl<E> Act for Link<E>
where
    E: 'static + Act + AddRoot + Update, // + Send + Sync,
{
    type Load = E::Load;
    fn act(&self) -> Self::Load {
        read_part(&self.edge, |edge|{
            let result = edge.act();
            edge.add_root(self.as_root());
            result
        })
        // let edge = self.edge.read().expect(NO_POISON);
        // let result = edge.act();
        // edge.add_root(self.as_root());
        // result
    }
}

impl<E> Solve for Link<E>
where
    E: 'static + Solve + AddRoot + Update, // + Send + Sync,
{
    type Task = E::Task;
    type Load = E::Load;
    fn solve(&self, task: Self::Task) -> Self::Load {
        read_part(&self.edge, |edge| {
            let result = edge.solve(task);
            edge.add_root(self.as_root());
            result
        })
        // let edge = self.edge.read().expect(NO_POISON);
        // let result = edge.solve(task);
        // edge.add_root(self.as_root());
        // result
    }
}

impl<E> Link<E>
where
    E: Grant + ToPloy<Load = <E as Grant>::Load>,
{
    pub fn ploy(&self) -> Ploy<<E as Grant>::Load> {
        read_part(&self.edge, |edge| {
            Ploy {
                edge: edge.ploy(),
                meta: self.meta.clone(),
            }
        })
        // let edge = self.edge.read().expect(NO_POISON);
        // Ploy {
        //     edge: edge.ploy(),
        //     meta: self.meta.clone(),
        // }
    }
}

impl<L> Backed for Link<Box<dyn Produce<L>>> { //  + Send + Sync
    fn backed(&self, back: &Back) -> Self {
        read_part(&self.edge, |edge| {
            Self {
                edge: edge.backed_ploy(back),
                meta: self.meta.clone(),
            }
        })
        // let edge = self.edge.read().expect(NO_POISON);
        // Self {
        //     edge: edge.backed_ploy(back),
        //     meta: self.meta.clone(),
        // }
    }
}

impl<E> Link<E>
where
    E: Solve + ToPlan<Task = <E as Solve>::Task, Load = <E as Solve>::Load>,
{
    pub fn plan(&self) -> Plan<<E as Solve>::Task, <E as Solve>::Load> {
        read_part(&self.edge, |edge| {
            Plan {
                edge: edge.plan(),
                meta: self.meta.clone(),
            }
        })
        // let edge = self.edge.read().expect(NO_POISON);
        // Plan {
        //     edge: edge.plan(),
        //     meta: self.meta.clone(),
        // }
    }
}

impl<T, L> Backed for Link<Box<dyn Convert<T, L>>> { //  + Send + Sync
    fn backed(&self, back: &Back) -> Self {
        read_part(&self.edge, |edge| {
            Self {
                edge: edge.backed_plan(back),
                meta: self.meta.clone(),
            }
        })
        // let edge = self.edge.read().expect(NO_POISON);
        // Self {
        //     edge: edge.backed_plan(back),
        //     meta: self.meta.clone(),
        // }
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

// struct EdgeGuard<'a, E> {
    //     edge: RwLockReadGuard<'a, E>,
    // }
    
    // fn read_edge<'a, E>(edge: Arc<RwLock<E>>) -> &'a EdgeGuard<'a, E> {
    //     let wow = edge.read(); //.expect(NO_POISON);
    //     &EdgeGuard {
    //         edge: wow.expect(NO_POISON)
    //     }
    // }

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
