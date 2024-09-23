use async_trait::async_trait;
// use async_trait::async_trait;
pub use leaf::*;

use super::*;
#[cfg(not(feature = "oneThread"))]
use parking_lot::RwLock;
#[cfg(not(feature = "oneThread"))]
use std::sync::Arc;
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
pub struct Link<E: ?Sized> {
    #[cfg(not(feature = "oneThread"))]
    edge: Arc<RwLock<E>>,
    #[cfg(feature = "oneThread")]
    edge: Rc<RefCell<E>>,
    path: Option<Path>,
    rank: Option<u64>,
}

impl<E: ?Sized> fmt::Debug for Link<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Path: {:?}", self.path))
    }
}

impl<U: 'static + Unit> Node<U> {
    pub fn hub(self) -> Hub<U::Base> {
        self.into()
    }
}

impl<T: Payload> Leaf<T> {
    pub fn hub(self) -> Hub<T> {
        self.into()
    }
}

impl<E: ?Sized> Link<E> {
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
    Self: Solve<Base = ()>,
    <Self as Solve>::Base: 'static + Payload,
{
    pub async fn act(&self) -> Result<()> {
        match self.solve().await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}

impl<E: ?Sized> Hash for Link<E>
where
    Self: Solve,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Some(path) = &self.path {
            path.hash(state)
        } else if let Ok(Gain::U64(hash)) = self.reckon(Task::Hash) {
            hash.hash(state)
        }
    }
}

impl<E: ?Sized> Serialize for Link<E>
where
    Self: Solve,
{
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Some(path) = &self.path {
            path.serialize(serializer)
        } else if let Ok(Gain::U64(hash)) = self.reckon(Task::Hash) {
            Path::Hash(hash).serialize(serializer)
        } else {
            serializer.serialize_str("ERROR(serialization)")
        }
    }
}

impl<E> Link<E>
where
    E: 'static + FromItem + SetRoot + Update,
{
    pub fn new(base: E::Item) -> Self {
        Self {
            path: None,
            rank: None,
            // TODO: find way to keep edge pointer function in edge file
            edge: edge_pointer(E::new(base)),
        }
    }
}

impl<E> Link<E>
where
    E: 'static + FromSnap + Update + SetRoot,
{
    pub fn from_unit(unit: E::Unit) -> Result<Self> {
        let (rank, edge) = E::from_snap(unit.into())?;
        Ok(Self {
            path: None,
            rank,
            edge,
        })
    }
}

impl<E> Link<E>
where
    E: 'static + FromSnap + Engage,
{
    // pub fn make_ploy<F: FnOnce(&Back) -> Result<E::Unit>>(make: F) -> Result<Ploy<E::Base>> {
    pub fn ploy_from_unit(unit: E::Unit) -> Result<Ploy<E::Base>> {
        let (rank, edge) = E::from_snap(unit.into())?;
        Ok(Link {
            path: None,
            rank,
            edge,
        })
    }
}

impl<E> Link<E>
where
    E: 'static + FromSnap + Engage,
{
    // TODO: add weak self to edge!!!
    pub fn ploy_from_snap(snap: Snap<E::Unit>) -> Result<Ploy<E::Base>> {
        let (rank, edge) = E::from_snap(snap)?;
        Ok(Link {
            path: None,
            rank,
            edge,
        })
    }
}

impl<E> Link<E>
where
    E: 'static + Engage,
{
    /// Copy the link with unit type erased.  
    pub fn to_ploy(&self) -> Ploy<E::Base> {
        Ploy {
            edge: self.edge.clone(),
            path: self.path.clone(),
            rank: self.rank,
        }
    }
}

impl<E: ?Sized> Clone for Link<E> {
    fn clone(&self) -> Self {
        Self {
            edge: self.edge.clone(),
            path: self.path.clone(),
            rank: self.rank,
        }
    }
}

impl<E: ?Sized> PartialEq for Link<E> {
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

impl<E: ?Sized> Backed for Link<E>
where
    E: 'static + BackedMid + Update,
{
    fn backed(&self, back: &Back) -> Result<Self> {
        read_part(&self.edge, |edge| Self {
            edge: edge.backed(back),
            path: self.path.clone(),
            rank: self.rank,
        })
    }
}

impl<T> Backed for Ploy<T>
where
    T: 'static + Payload,
{
    fn backed(&self, back: &Back) -> Result<Self> {
        read_part(&self.edge, |edge| {
            Ok(Self {
                edge: edge.backed(back),
                path: self.path.clone(),
                rank: self.rank,
            })
        })?
    }
}

impl<E> Link<E>
where
    E: 'static + Read + Update
{
    /// Read payload of Link.
    pub fn read<O, F: FnOnce(&E::Item) -> O>(&self, read: F) -> Result<O> {
        read_part(&self.edge, |edge| edge.read(read))?
    }
}

#[cfg_attr(not(feature = "oneThread"), async_trait)]
#[cfg_attr(feature = "oneThread", async_trait(?Send))]
impl<E, T> WriteBase<T> for Link<E>
where
    E: WriteBase<T> + SendSync,
{
    async fn write<O: SendSync, F: FnOnce(&mut T) -> O + SendSync>(&self, write: F) -> Result<O> {
        read_part_async(&self.edge, |edge| async move { edge.write(write).await })?.await
    }
}

#[cfg_attr(not(feature = "oneThread"), async_trait)]
#[cfg_attr(feature = "oneThread", async_trait(?Send))]
impl<E> WriteUnit for Link<E>
where
    E: WriteUnit + SendSync,
{
    type Unit = E::Unit;
    async fn write<O: SendSync, F: FnOnce(&mut Pack<Self::Unit>) -> O + SendSync>(
        &self,
        write: F,
    ) -> Result<O> {
        read_part_async(&self.edge, |edge| async move { edge.write(write).await })?.await
    }
}

impl<E> Solve for Link<E>
where
    E: 'static + Solve + Update,
    E::Base: Payload,
{
    type Base = E::Base;
    async fn solve(&self) -> Result<Hub<Self::Base>> {
        read_part_async(&self.edge, |edge| async move { edge.solve().await })?.await
    }
    fn reckon(&self, task: Task) -> Result<Gain> {
        read_part(&self.edge, |edge| edge.reckon(task))?
    }
}

impl<T> Solve for Ploy<T>
where
    T: 'static + Payload,
{
    type Base = T;
    async fn solve(&self) -> Result<Hub<Self::Base>> {
        read_part_async(&self.edge, |edge| async move { edge.solve().await })?.await
    }
    fn reckon(&self, task: Task) -> Result<Gain> {
        read_part(&self.edge, |edge| edge.reckon(task))?
    }
}

impl<E: ?Sized> AdaptGet for Link<E>
where
    E: 'static + AdaptGet + Update
{
    fn adapt_get(&self, deal: &mut dyn Deal) -> Result<()> {
        read_part(&self.edge, |edge| {
            edge.adapt_get(deal)
            // if deal.read() {
            //     edge.add_root(self.as_root(edge.id()))?;
            // }
            // } else {
            //     return Err(anyhow!("Deal did not report reading in AdaptGet"))?;
            // }
            // if deal.wrote() {
            //     return Err(anyhow!("Deal should not write in AdaptGet"))?;
            // }
            // out
        })?
    }
}

#[cfg_attr(not(feature = "oneThread"), async_trait)]
#[cfg_attr(feature = "oneThread", async_trait(?Send))]
impl<E: ?Sized> AdaptSet for Link<E>
where
    E: 'static + AdaptSet + Update
{
    async fn adapt_set(&self, deal: &mut dyn Deal) -> Result<()> {
        read_part_async(&self.edge, |edge| async move {
            edge.adapt_set(deal).await
            // if deal.read() {
            //     return Err(anyhow!("Deal should not read in AdaptSet"))?;
            // }
            // if !deal.wrote() {
            //     return Err(anyhow!("Deal did not report writing in AdaptSet"))?;
            // }
            // result
        })?
        .await
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