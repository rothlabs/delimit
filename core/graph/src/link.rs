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
    edge: Pointer<E>,
    path: Option<Path>,
    rank: Option<u16>,
}

impl<E: ?Sized> fmt::Debug for Link<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Path: {:?}", self.path))
    }
}

impl<U: 'static + Unit + HashGraph + Serialize> Node<U> {
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
    pub fn rank(&self) -> Option<u16> {
        self.rank
    }
}

impl<E> Link<E> 
where 
    E: Reckon + ?Sized
{
    pub fn get_imports(&self) -> Result<Vec<Import>> {
        read_part(&self.edge, |edge| edge.get_imports())?
    }
    pub fn get_hash(&self) -> Result<u64> {
        read_part(&self.edge, |edge| edge.get_hash())?
    }
    pub fn get_serial(&self) -> Result<String> {
        read_part(&self.edge, |edge| edge.get_serial())?
    }
}

// impl<E> Link<E>
// where
//     Self: SolveLink,
// {
//     pub async fn act(&self) -> Result<()> {
//         match self.solve().await {
//             Ok(_) => Ok(()),
//             Err(err) => Err(err),
//         }
//     }
// }

impl<E> HashGraph for Link<E>
where
    E: Reckon + ?Sized,
{
    fn hash_graph<H: Hasher>(&self, state: &mut H) {
        if let Some(path) = &self.path {
            path.hash(state);
        } else if let Ok(Ok(hash)) = read_part(&self.edge, |edge| edge.get_hash()) {//if let Ok(Gain::U64(hash)) = self.reckon(Task::Hash) {
            hash.hash_graph(state);
            // read_part(&self.edge, |edge| edge.get_hash()).unwrap().unwrap().hash_graph(state)
        }
    }
}

impl<E> Serialize for Link<E>
where
    E: Reckon + ?Sized
{
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Some(path) = &self.path {
            path.serialize(serializer)
        } else if let Ok(Ok(hash)) = read_part(&self.edge, |edge| edge.get_hash()) {// Ok(Gain::U64(hash)) = self.reckon(Task::Hash) {
            Path::Hash(hash).serialize(serializer)
        } else {
            serializer.serialize_str("ERROR(serialization)")
        }
    }
}

impl<E: FromBase> Link<E> {
    pub fn new(base: E::Base) -> Self {
        Self {
            path: None,
            rank: None,
            edge: E::from_base(base),
        }
    }
}

impl<E: FromSnap> Link<E> {
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
    pub fn ploy_from_unit(unit: E::Unit) -> Result<Ploy<E::Base>> {
        let (rank, edge) = E::from_snap(unit.into())?;
        Ok(Ploy {
            path: None,
            rank,
            edge,
        })
    }
}

impl<E> Link<E>
where
    E: 'static + FromSnap + Employ,
{
    pub fn wing_from_unit(unit: E::Unit) -> Result<Wing<E::Base>> {
        let (rank, edge) = E::from_snap(unit.into())?;
        Ok(Wing {
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
    pub fn ploy_from_snap(snap: Snap<E::Unit>) -> Result<Ploy<E::Base>> {
        let (rank, edge) = E::from_snap(snap)?;
        Ok(Ploy {
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
    pub fn as_ploy(&self) -> Ploy<E::Base> {
        Ploy {
            edge: self.edge.clone(),
            path: self.path.clone(),
            rank: self.rank,
        }
    }
}

impl<E> Link<E>
where
    E: 'static + Employ,
{
    /// Copy the link with unit type erased.  
    pub fn wing(&self) -> Wing<E::Base> {
        Wing {
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
    fn eq(&self, rhs: &Self) -> bool {
        #[cfg(not(feature = "oneThread"))]
        let ptr_eq = Arc::<RwLock<E>>::ptr_eq(&self.edge, &rhs.edge);
        #[cfg(feature = "oneThread")]
        let ptr_eq = Rc::<RefCell<E>>::ptr_eq(&self.edge, &rhs.edge);
        ptr_eq && self.path == rhs.path && self.rank == rhs.rank
    }
}

impl<E> Backed for Link<E>
where
    E: BackedMid + ?Sized,
{
    fn backed(&self, back: &Back) -> Result<Self> {
        read_part(&self.edge, |edge| Self {
            edge: edge.backed(back),
            path: self.path.clone(),
            rank: self.rank,
        })
    }
}

impl<T: Payload> Backed for Ploy<T> {
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

impl<T: Payload> Backed for Wing<T> {
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

impl<E: Read> Link<E> {
    /// Read payload of Link.
    pub fn read<F, O>(&self, read: F) -> Result<O>
    where
        // TODO: take ReadGuard directly so its lifetime is okay for async block in closure
        F: FnOnce(&E::Item) -> O,
    {
        read_part(&self.edge, |edge| edge.read(read))?
    }
}

impl<E> WriteBase for Link<E>
where
    E: WriteBase + SendSync,
{
    type Base = E::Base;
    async fn write<O, F>(&self, write: F) -> Result<O>
    where
        O: IsSend,
        F: FnOnce(&mut E::Base) -> O + IsSend,
    {
        read_part(&self.edge, |edge| async move { edge.write(write).await })?.await
    }
}

impl<E> WriteUnit for Link<E>
where
    E: WriteUnit + SendSync,
{
    type Unit = E::Unit;
    async fn write<O, F>(&self, write: F) -> Result<O>
    where
        O: IsSend,
        F: FnOnce(&mut Pack<E::Unit>) -> O + IsSend,
    {
        read_part(&self.edge, |edge| async move { edge.write(write).await })?.await
    }
}

impl<E: Solve> Link<E> {
    pub async fn solve(&self) -> Result<Hub<E::Base>> {
        read_part(&self.edge, |edge| async move { edge.solve().await })?.await
    }
    pub async fn act(&self) -> Result<()> {
        match self.solve().await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}

impl<T: Payload> Ploy<T> {
    pub async fn solve(&self) -> Result<Hub<T>> {
        read_part(&self.edge, |edge| async move { edge.solve().await })?.await
    }
}

impl<T: Payload> Wing<T> {
    pub async fn solve(&self) -> Result<Hub<T>> {
        read_part(&self.edge, |edge| async move { edge.solve().await })?.await
    }
}

impl<E> Adapt for Link<E>
where
    E: Adapt + ?Sized + SendSync,
{
    fn adapt_get(&self, deal: &mut dyn Deal) -> Result<()> {
        read_part(&self.edge, |edge| edge.adapt_get(deal))?
    }
    fn adapt_set<'a>(&'a self, deal: &'a mut dyn Deal) -> GraphFuture<Result<()>> {
        Box::pin(async move {
            read_part(&self.edge, |edge| async move { edge.adapt_set(deal).await })?.await
        })
    }
    fn transient_set(&self, deal: &mut dyn Deal) -> Result<Ring> {
        read_part(&self.edge, |edge| edge.transient_set(deal))?
    }
}

impl<T: Backed> Backed for Vec<T> {
    fn backed(&self, back: &Back) -> Result<Self> {
        self.iter().map(|link| link.backed(back)).collect()
    }
}

impl<T: Backed> Backed for Option<T> {
    fn backed(&self, back: &Back) -> Result<Self> {
        if let Some(x) = self {
            Ok(Some(x.backed(back)?))
        } else {
            Ok(None)
        }
    }
}
