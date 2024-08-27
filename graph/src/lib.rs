pub use adapt::{adapt_ok, no_adapter, Adapt, AdaptMid, AdaptOut, Memo, Post};
pub use apex::{Apex, EngageApexes};
pub use bay::Bay;
pub use cusp::Cusp;
pub use edge::Edge;
pub use lake::{Lake, SerialNode};
pub use link::{Leaf, Link, Node, Ploy, ToLeaf};
pub use meta::{random, upper_all, Id, Import, Key, Path, ToId, WORLD_ALL};
pub use react::{
    AddRoot, AddRootMut, Back, Backed, BackedPloy, React, ReactMut, Rebut, RebutMut, Ring, Root,
    ToPipedPloy, ToPloy, Update, UpdateMid,
};
pub use serial::{DeserializeUnit, ToHash, ToSerial, UnitHasher};
pub use snap::{IntoSnapWithImport, IntoSnapWithImports, Snap};
pub use solve::{empty_apexes, no_solver, solve_ok, DoSolve, Gain, IntoGain, Solve, Task};
pub use tray::Tray;
pub use write::{
    Pack, WriteTray, WriteTrayOut, WriteTrayWork, WriteUnit, WriteUnitOut, WriteUnitWork,
};

use dyn_clone::DynClone;
use scope::*;
use serde::{Deserialize, Serialize};
#[cfg(not(feature = "oneThread"))]
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
#[cfg(feature = "oneThread")]
use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};
use std::{
    collections::{hash_map::Iter, HashMap},
    error,
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
};

pub mod adapt;
pub mod apex;
pub mod lake;
pub mod react;
pub mod serial;
pub mod solve;
pub mod work;

mod bay;
mod cusp;
mod edge;
mod link;
mod meta;
mod scope;
mod snap;
mod tray;
mod write;

#[cfg(not(feature = "oneThread"))]
/// Graph Error
pub type Error = Box<dyn error::Error + Send + Sync>;
#[cfg(feature = "oneThread")]
/// Graph Error
pub type Error = Box<dyn error::Error>;

#[cfg(not(feature = "oneThread"))]
const NO_POISON: &str = "The lock should not be poisoned.";

#[cfg(not(feature = "oneThread"))]
pub trait SendSync: Send + Sync {}
#[cfg(not(feature = "oneThread"))]
impl<T: Send + Sync> SendSync for T {}

#[cfg(feature = "oneThread")]
pub trait SendSync {}
#[cfg(feature = "oneThread")]
impl<T> SendSync for T {}

#[cfg(not(feature = "oneThread"))]
fn read_part<P: ?Sized, O, F: FnOnce(RwLockReadGuard<P>) -> O>(
    part: &Arc<RwLock<P>>,
    read: F,
) -> O {
    read(part.read().expect(NO_POISON))
}

#[cfg(feature = "oneThread")]
fn read_part<P: ?Sized, O, F: FnOnce(Ref<P>) -> O>(part: &Rc<RefCell<P>>, read: F) -> O {
    read(part.borrow())
}

#[cfg(not(feature = "oneThread"))]
fn write_part<P: ?Sized, O, F: FnOnce(RwLockWriteGuard<P>) -> O>(
    part: &Arc<RwLock<P>>,
    write: F,
) -> O {
    write(part.write().expect(NO_POISON))
}

#[cfg(feature = "oneThread")]
fn write_part<P: ?Sized, O, F: FnOnce(RefMut<P>) -> O>(part: &Rc<RefCell<P>>, write: F) -> O {
    write(part.borrow_mut())
}

/// General engagement of Ploy with erased unit type.
pub trait Engage: Solve + AdaptMid + BackedPloy + AddRoot + Update + Debug {}

#[cfg(not(feature = "oneThread"))]
type PloyEdge = Arc<RwLock<Box<dyn Engage>>>;
#[cfg(feature = "oneThread")]
type PloyEdge = Rc<RefCell<Box<dyn Engage>>>;

// dyn_clone::clone_trait_object!(Trade);
/// Trade a apex for another.
/// The implmentation should return the same semantic apex with different graph qualities.
pub trait Trade: Debug {
    // DynClone +
    /// Trade a apex for another.
    fn trade(&self, apex: &Apex) -> Apex;
}

pub trait IntoNode
where
    Self: Sized,
{
    fn node(self) -> Node<Self>;
}

impl<T> IntoNode for T
where
    T: 'static + Adapt + Solve + SendSync + Debug,
{
    fn node(mut self) -> Node<Self> {
        Node::make(|back| {
            self.adapt(Post::Trade(back))
                .expect("To move into Node, unit must Adapt with Post::Trade.");
            self
        })
    }
}

pub trait IntoApex {
    /// Move into `Apex`
    fn apex(self) -> Apex;
}

impl<T> IntoApex for T
where
    T: 'static + IntoNode + Solve + Adapt + Debug + SendSync,
{
    fn apex(self) -> Apex {
        self.node().ploy().into()
    }
}

impl IntoApex for Leaf {
    fn apex(self) -> Apex {
        self.into()
    }
}

pub trait ToApex {
    /// Place inside a Apex.
    fn apex(&self) -> Apex;
}

impl<T> ToApex for Node<T>
where
    T: 'static + Solve + Adapt + Debug + SendSync,
{
    fn apex(&self) -> Apex {
        self.ploy().into()
    }
}

pub trait ReadMid {
    type Item;
    fn read(&self) -> &Self::Item;
}

pub trait Read {
    type Item;
    /// Read the unit of the node.
    fn read<T, F: FnOnce(&Self::Item) -> T>(&self, read: F) -> T;
}

pub trait ReadTray {
    fn read_tray<T, F: FnOnce(tray::ResultRef) -> T>(&self, read: F) -> T;
}

pub trait ReadTrayMid {
    fn read_tray(&self) -> tray::ResultRef;
}

pub trait ToTray {
    type Tray;
    /// Clone the Tray out of the graph part.
    /// May cause stem apexes to generate the tray.
    fn tray(&self) -> Self::Tray;
}

pub trait FromItem {
    type Item;
    fn new(item: Self::Item) -> Self;
}

pub trait Make {
    type Unit;
    fn make<F: FnOnce(&Back) -> Self::Unit>(make: F) -> Self;
}

pub trait MakeMid {
    type Unit;
    fn make<F: FnOnce(&Back) -> Self::Unit>(&mut self, make: F, back: &Back);
}

pub trait FromSnap {
    type Unit;
    fn from_snap(snap: Snap<Self::Unit>) -> Self;
}

pub trait FromSnapMid {
    type Unit;
    fn from_snap(&mut self, snap: Snap<Self::Unit>, back: &Back);
}

pub trait Clear {
    fn clear(&mut self);
}

/// Key-Apex map.
#[derive(Default, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Map(HashMap<Key, Apex>);

impl Map {
    pub fn insert(&mut self, key: Key, apex: Apex) {
        self.0.insert(key, apex);
    }
    pub fn extend(&mut self, other: Map) {
        self.0.extend(other.0);
    }
    pub fn trade(&self, deal: &dyn Trade) -> Self {
        let mut map = HashMap::new();
        for (key, apex) in &self.0 {
            map.insert(key.clone(), apex.deal(deal));
        }
        Map(map)
    }
    pub fn get(&self, key: &Key) -> Option<Apex> {
        self.0.get(key).map(|apex| apex.pathed(key))
    }
    pub fn vec(&self) -> Vec<Apex> {
        let mut out = vec![];
        for (key, apex) in &self.0 {
            out.push(apex.pathed(key));
        }
        out
        // self.0.values().cloned().collect()
    }
    pub fn iter(&self) -> Iter<String, Apex> {
        self.0.iter()
    }
}

impl Hash for Map {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut pairs: Vec<_> = self.0.iter().collect();
        pairs.sort_by_key(|i| i.0);
        Hash::hash(&pairs, state);
    }
}

// impl<T> ToApex for T
// where
//     T: 'static + ToNode + Solve + Adapt + Debug + SendSync,
// {
//     fn apex(&self) -> Apex {
//         self.node().ploy().into()
//     }
// }

// if let Some(apex) = self.0.get(key) {
//     Some(apex.pathed(key))
// } else {
//     None
// }
