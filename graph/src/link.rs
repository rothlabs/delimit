use async_trait::async_trait;
pub use leaf::*;

use futures::executor::block_on;
use anyhow::anyhow;
use super::*;
#[cfg(not(feature = "oneThread"))]
use std::sync::{Arc, RwLock};
#[cfg(feature = "oneThread")]
use std::{cell::RefCell, rc::Rc};
use std::{
    fmt,
    hash::{Hash, Hasher},
};

mod leaf;
#[cfg(test)]
mod tests;

/// `Link` to `Tray`.
pub type Leaf<T> = Link<edge::Leaf<T>>;

/// `Link` to domain-specific unit.
/// The unit type is intact. For type-erased unit, use `Ploy` instead.
pub type Node<U> = Link<edge::Node<U>>;

/// `Link` to `Edge`, pointing to `Cusp`, containing work unit.
/// Unit fields often contain `Link`, creating a graph pattern.
pub struct Link<E> {
    #[cfg(not(feature = "oneThread"))]
    edge: Arc<RwLock<E>>,
    #[cfg(feature = "oneThread")]
    edge: Rc<RefCell<E>>,
    path: Option<Path>,
    rank: Option<u64>,
}

impl<E> fmt::Debug for Link<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Path: {:?}", self.path))
    }
}

impl<E> Link<E> {
    pub fn pathed(&self, path: Path) -> Self {
        Self {
            edge: self.edge.clone(),
            path: Some(path),
            rank: self.rank,
        }
    }
    pub fn rank(&self) -> Option<u64> {
        self.rank
    }
}

impl<E> Link<E>
where
    Self: Solve,
    <Self as Solve>::Base: 'static + Payload,
{
    pub async fn main(&self) -> Result<Hub<<Self as Solve>::Base>> {
        // let wow = self.solve(Task::Main);
        match self.solve(Task::Main).await? {
            Gain::Hub(hub) => Ok(hub),
            _ => Err(anyhow!("Wrong return type for Task::Main."))?,
        }
    }
    pub fn main_block(&self) -> Result<Hub<<Self as Solve>::Base>> {
        // let wow = self.solve(Task::Main);
        match block_on(self.solve(Task::Main))? {
            Gain::Hub(hub) => Ok(hub),
            _ => Err(anyhow!("Wrong return type for Task::Main."))?,
        }
    }
    pub async fn act(&self) -> Result<()> {
        match self.solve(Task::None).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}

impl<E> Hash for Link<E>
where
    Self: Solve,
    <Self as Solve>::Base: 'static + Payload,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Some(path) = &self.path {
            path.hash(state)
        } else if let Ok(Gain::U64(hash)) = block_on(self.solve(Task::Hash)) {
            hash.hash(state)
        }
    }
}

impl<E> Serialize for Link<E>
where
    Self: Solve,
    <Self as Solve>::Base: 'static + Payload,
{
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Some(path) = &self.path {
            path.serialize(serializer)
        } else if let Ok(Gain::U64(hash)) = block_on(self.solve(Task::Hash)) {
            Path::Hash(hash).serialize(serializer)
        } else {
            serializer.serialize_str("ERROR(serialization)")
        }
    }
}

impl<E> Link<E>
where
    E: FromItem,
{
    pub fn new(unit: E::Item) -> Self {
        let edge = E::new(unit);
        Self {
            path: None,
            rank: None,
            #[cfg(not(feature = "oneThread"))]
            edge: Arc::new(RwLock::new(edge)),
            #[cfg(feature = "oneThread")]
            edge: Rc::new(RefCell::new(edge)),
        }
    }
}

impl<E> Link<E>
where
    E: Make,
{
    pub fn make<F: FnOnce(&Back) -> Result<E::Unit>>(make: F) -> Result<Self> {
        let (edge, rank) = E::make(make)?;
        Ok(Self {
            path: None,
            rank,
            #[cfg(not(feature = "oneThread"))]
            edge: Arc::new(RwLock::new(edge)),
            #[cfg(feature = "oneThread")]
            edge: Rc::new(RefCell::new(edge)),
        })
    }
}

impl<E> Link<E>
where
    E: 'static + Make + Engage,
{
    pub fn make_ploy<F: FnOnce(&Back) -> Result<E::Unit>>(make: F) -> Result<Ploy<E::Base>> {
        let (edge, rank) = E::make(make)?;
        Ok(Link {
            path: None,
            rank,
            #[cfg(not(feature = "oneThread"))]
            edge: Arc::new(RwLock::new(
                Box::new(edge) as Box<dyn Engage<Base = E::Base>>
            )),
            #[cfg(feature = "oneThread")]
            edge: Rc::new(RefCell::new(
                Box::new(edge) as Box<dyn Engage<Base = E::Base>>
            )),
        })
    }
}

impl<E> Link<E>
where
    E: ToPloy,
{
    /// Copy the link with unit type erased.  
    pub fn ploy(&self) -> Result<Ploy<E::Base>> {
        read_part(&self.edge, |edge| Ploy {
            edge: edge.ploy(),
            path: self.path.clone(),
            rank: self.rank,
        })
    }
}

impl<E> Link<E>
where
    E: 'static + FromSnap + Engage,
{
    pub fn make_ploy_from_snap(snap: Snap<E::Unit>) -> Ploy<E::Base> {
        let (edge, rank) = E::from_snap(snap);
        Link {
            path: None,
            rank,
            #[cfg(not(feature = "oneThread"))]
            edge: Arc::new(RwLock::new(
                Box::new(edge) as Box<dyn Engage<Base = E::Base>>
            )),
            #[cfg(feature = "oneThread")]
            edge: Rc::new(RefCell::new(
                Box::new(edge) as Box<dyn Engage<Base = E::Base>>
            )),
        }
    }
}

impl<E> Link<E>
where
    E: 'static + Update,
{
    #[cfg(not(feature = "oneThread"))]
    pub fn as_root(&self, id: Id) -> Root {
        let edge = self.edge.clone() as Arc<RwLock<dyn Update>>;
        Root {
            edge: Arc::downgrade(&edge),
            id,
        }
    }
    #[cfg(feature = "oneThread")]
    pub fn as_root(&self, id: Id) -> Root {
        let edge = self.edge.clone() as Rc<RefCell<dyn Update>>;
        Root {
            edge: Rc::downgrade(&edge),
            id,
        }
    }
}

impl<E> Clone for Link<E> {
    fn clone(&self) -> Self {
        Self {
            edge: self.edge.clone(),
            path: self.path.clone(),
            rank: self.rank,
        }
    }
}

impl<E> PartialEq for Link<E> {
    #[cfg(not(feature = "oneThread"))]
    fn eq(&self, other: &Self) -> bool {
        Arc::<RwLock<E>>::ptr_eq(&self.edge, &other.edge)
            && self.path == other.path
            && self.rank == other.rank
    }
    #[cfg(feature = "oneThread")]
    fn eq(&self, other: &Self) -> bool {
        Rc::<RefCell<E>>::ptr_eq(&self.edge, &other.edge)
            && self.path == other.path
            && self.rank == other.rank
    }
}

impl<E> Backed for Link<E>
where
    E: BackedMid,
{
    fn backed(&self, back: &Back) -> Result<Self> {
        read_part(&self.edge, |edge| {
            #[cfg(not(feature = "oneThread"))]
            let edge = Arc::new(RwLock::new(edge.backed(back)));
            #[cfg(feature = "oneThread")]
            let edge = Rc::new(RefCell::new(edge.backed(back)));
            Ok(Self {
                edge,
                path: self.path.clone(),
                rank: self.rank,
            })
        })?
    }
}

impl<T> Backed for Ploy<T>
where
    T: 'static + Payload,
{
    fn backed(&self, back: &Back) -> Result<Self> {
        read_part(&self.edge, |edge| Self {
            edge: edge.backed(back),
            path: self.path.clone(),
            rank: self.rank,
        })
    }
}

impl<E> Link<E>
where
    E: 'static + Read + Update + AddRoot,
{
    /// Read payload of Link.
    pub fn read<O, F: FnOnce(&E::Item) -> O>(&self, read: F) -> Result<O> {
        read_part(&self.edge, |edge| {
            let out = edge.read(read);
            edge.add_root(self.as_root(edge.id()))?;
            out
        })?
    }
}

impl<E, T> WriteBase<T> for Link<E>
where
    E: WriteBase<T>,
{
    fn write<O, F: FnOnce(&mut T) -> O>(&self, write: F) -> Result<O> {
        read_part(&self.edge, |edge| edge.write(write))?
    }
}

impl<E> WriteUnit for Link<E>
where
    E: WriteUnit,
{
    type Unit = E::Unit;
    fn write<O, F: FnOnce(&mut Pack<Self::Unit>) -> O>(&self, write: F) -> Result<O> {
        read_part(&self.edge, |edge| edge.write(write))?
    }
}

// #[async_trait(?Send)]
impl<E> Solve for Link<E>
where
    E: 'static + Solve + AddRoot + Update,
    E::Base: Payload,
{
    type Base = E::Base;
    async fn solve(&self, task: Task<'_>) -> Result<Gain<Self::Base>> {
        match self.edge.read() {
            Ok(edge) => {
                let result = edge.solve(task).await;
                edge.add_root(self.as_root(edge.id()))?;
                result
            },
            Err(err) => Err(Error::Read(err.to_string())),
        }
        // let wow = read_part(&self.edge, |edge| {
        //     let fut = async {
        //         let result = edge.solve(task).await;
        //         edge.add_root(self.as_root(edge.id()))?;
        //         result
        //     };
        //     fut
        // })?;
        // wow
    }
}

impl<T> Solve for Ploy<T>
where
    T: 'static + Payload,
{
    type Base = T;
    async fn solve(&self, task: Task<'_>) -> Result<Gain<Self::Base>> {
        match self.edge.read() {
            Ok(edge) => {
                let result = edge.solve(task).await;
                edge.add_root(self.as_root(edge.id()))?;
                result
            },
            Err(err) => Err(Error::Read(err.to_string())),
        }
        // let wow = read_part(&self.edge, |edge| {
        //     async {
        //         let result = edge.solve(task).await;
        //         edge.add_root(self.as_root(edge.id()))?;
        //         result
        //     }
        // })?;
        // wow
    }
}

impl<E> AdaptMid for Link<E>
where
    E: 'static + AdaptMid + AddRoot + Update,
{
    fn adapt(&self, deal: &mut dyn Deal) -> Result<()> {
        read_part(&self.edge, |edge| {
            let result = edge.adapt(deal);
            if deal.read() {
                edge.add_root(self.as_root(edge.id()))?;
            }
            result
        })?
    }
}

impl<T> Backed for Vec<T>
where
    T: Backed,
{
    fn backed(&self, back: &Back) -> Result<Self> {
        self.iter().map(|link| link.backed(back)).collect()
    }
}

impl<T> Backed for Option<T>
where
    T: Backed,
{
    fn backed(&self, back: &Back) -> Result<Self> {
        if let Some(x) = self {
            Ok(Some(x.backed(back)?))
        } else {
            Ok(None)
        }
    }
}
