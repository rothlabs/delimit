pub use leaf::*;

use super::*;
// #[cfg(not(feature = "oneThread"))]
// use parking_lot::RwLock;
#[cfg(not(feature = "oneThread"))]
use std::sync::Arc;
// #[cfg(feature = "oneThread")]
// use std::{cell::RefCell, rc::Rc};
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

/// `Link` to domain-specific node.
/// The unit type is erased. To keep unit type intact, use `Node` instead.
pub type Ploy<T> = Link<dyn edge::ploy::Engage<Base = T>>;

pub type Gate<T> = Link<dyn edge::gate::Engage<Base = T>>;

/// `Link` to `Edge`, pointing to `Cusp`, containing work unit.
/// Unit fields often contain `Link`, creating a graph pattern.
// #[derive(Default)]
pub struct Link<E: ?Sized> {
    edge: Grc<E>,
    root: Root,
    // TODO: instead of option path: there could be a PathedLink that has a path and Link
    path: Option<Path>,
    // TODO: should be able to get rid of rank
    rank: Option<u16>,
}

impl<E: edge::FromBase> Default for Link<E> 
where 
    E::Base: Default
{
    fn default() -> Self {
        let (edge, root) = E::from_base(E::Base::default());
        Self {
            edge,
            root,
            path: None,
            rank: None,
        }
    }
}

impl<E: ?Sized> fmt::Debug for Link<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Path: {:?}", self.path))
    }
}

// fn root_edge<E: 'static + Update>(edge: &Grc<E>) -> Root {
//     let update = edge.clone() as Grc<dyn Update>;
//     Root {
//         edge: Grc::downgrade(&update),
//         id: rand::random(),
//     }
// }

impl<T: Clone> Leaf<T> {
    pub fn hub(self) -> Hub<T> {
        self.into()
    }
    pub fn base(&self) -> Result<T> {
        self.read(|base| base.clone())
    }
}

impl<E: ?Sized> Link<E> {
    pub fn pathed(&self, path: Path) -> Self {
        Self {
            root: self.root.clone(),
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
    E: Reckon + ?Sized,
{
    pub fn get_imports(&self) -> Result<Vec<Import>> {
        // read_part(&self.edge, |edge| edge.get_imports())?
        self.edge.get_imports()
    }
    pub fn get_hash(&self) -> Result<u64> {
        // read_part(&self.edge, |edge| edge.get_hash())?
        self.edge.get_hash()
    }
    pub fn get_serial(&self) -> Result<String> {
        // read_part(&self.edge, |edge| edge.get_serial())?
        self.edge.get_serial()
    }
}

impl<E> Digest for Link<E>
where
    E: Reckon + ?Sized,
{
    fn digest<H: Hasher>(&self, state: &mut H) {
        if let Some(path) = &self.path {
            path.hash(state);
        } else if let Ok(hash) = self.edge.get_hash() {
            //if let Ok(Gain::U64(hash)) = self.reckon(Task::Hash) {
            hash.digest(state);
            // read_part(&self.edge, |edge| edge.get_hash()).unwrap().unwrap().digest(state)
        }
    }
}

impl<E> Serialize for Link<E>
where
    E: Reckon + ?Sized,
{
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Some(path) = &self.path {
            path.serialize(serializer)
        } else if let Ok(hash) = self.edge.get_hash() {
            // Ok(Gain::U64(hash)) = self.reckon(Task::Hash) {
            Path::Hash(hash).serialize(serializer)
        } else {
            serializer.serialize_str("ERROR(serialization)")
        }
    }
}

impl<E: edge::FromBase> Link<E> {
    pub fn new(base: E::Base) -> Self {
        let (edge, root) = E::from_base(base);
        Self {
            path: None,
            rank: None,
            edge,//: E::from_base(base),
            root,
        }
    }
}

impl<E: edge::FromSnap> Link<E> {
    pub fn from_unit(unit: E::Unit) -> Self {
        let (rank, edge, root) = E::from_snap(unit.into());
        Self {
            path: None,
            rank,
            edge,
            root,
        }
    }
}

impl<E> Link<E>
where
    E: 'static + edge::FromSnap + ploy::Engage,
{
    pub fn ploy_from_unit(unit: E::Unit) -> Ploy<E::Base> {
        let (rank, edge, root) = E::from_snap(unit.into());
        Ploy {
            path: None,
            rank,
            edge,
            root,
        }
    }
}

impl<E> Link<E>
where
    E: 'static + edge::FromSnap + gate::Engage,
{
    pub fn gate_from_unit(unit: E::Unit) -> Gate<E::Base> {
        let (rank, edge, root) = E::from_snap(unit.into());
        Gate {
            path: None,
            rank,
            edge,
            root,
        }
    }
}

impl<E> Link<E>
where
    E: 'static + edge::FromSnap + ploy::Engage,
{
    pub fn ploy_from_snap(snap: Snap<E::Unit>) -> Ploy<E::Base> {
        let (rank, edge, root) = E::from_snap(snap);
        Ploy {
            path: None,
            rank,
            edge,
            root,
        }
    }
}

impl<E: ?Sized> Clone for Link<E> {
    fn clone(&self) -> Self {
        Self {
            root: self.root.clone(),
            edge: self.edge.clone(),
            path: self.path.clone(),
            rank: self.rank,
        }
    }
}

impl<E: ?Sized> PartialEq for Link<E> {
    fn eq(&self, rhs: &Self) -> bool {
        #[cfg(not(feature = "oneThread"))]
        let ptr_eq = Arc::<E>::ptr_eq(&self.edge, &rhs.edge);
        #[cfg(feature = "oneThread")]
        let ptr_eq = Rc::<E>::ptr_eq(&self.edge, &rhs.edge);
        ptr_eq && self.path == rhs.path && self.rank == rhs.rank
    }
}

// TODO: impl Backed for Leaf<U> so SendSync is not needed for Backed Hub?
impl<E> Backed for Link<E>
where
    E: edge::BackedMid + ?Sized,
{
    fn backed(&self, back: &Back) -> Self {
        let (edge, root) = self.edge.backed(back);
        Self {
            edge,
            root,
            path: self.path.clone(),
            rank: self.rank,
        }
    }
}

impl<T> Backed for Ploy<T> {
    fn backed(&self, back: &Back) -> Self {
        let (edge, root) = self.edge.backed(back);
        Self {
            edge,
            root,
            path: self.path.clone(),
            rank: self.rank,
        }
    }
}

impl<T> Backed for Gate<T> {
    fn backed(&self, back: &Back) -> Self {
        let (edge, root) = self.edge.backed(back);
        Self {
            edge,
            root,
            path: self.path.clone(),
            rank: self.rank,
        }
    }
}

impl<E: edge::Read> Link<E> {
    /// Read payload of Link.
    pub fn read<F, O>(&self, read: F) -> Result<O>
    where
        // TODO: take ReadGuard directly so its lifetime is okay for async block in closure
        F: FnOnce(&E::Item) -> O,
    {
        self.edge.read(read, self.root.clone())
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
        // read_part(&self.edge, |edge| async move { edge.write(write).await })?.await
        self.edge.write(write).await
    }
    fn write_passive<O, F>(&self, write: F) -> Result<O>
    where
        O: IsSend,
        F: FnOnce(&mut Self::Base) -> O + IsSend,
    {
        // read_part(&self.edge, |edge| edge.write_passive(write))?
        self.edge.write_passive(write)
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
        // read_part(&self.edge, |edge| async move { edge.write(write).await })?.await
        self.edge.write(write).await
    }
}

impl<E: edge::Solve> Link<E> {
    pub async fn solve(&self) -> Result<Hub<E::Base>> {
        // Ok(read_part(&self.edge, |edge| async move { edge.solve().await })?.await?)
        Ok(self.edge.solve(self.root.clone()).await?)
    }
    pub async fn act(&self) -> Result<()> {
        match self.solve().await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}

impl<T: SendSync> Ploy<T> {
    pub async fn solve(&self) -> Result<Hub<T>> {
        // read_part(&self.edge, |edge| async move { edge.solve().await })?.await
        self.edge.solve(self.root.clone()).await
    }
}

impl<T: SendSync> Gate<T> {
    pub async fn solve(&self) -> Result<Hub<T>> {
        // read_part(&self.edge, |edge| async move { edge.solve().await })?.await
        self.edge.solve(self.root.clone()).await
    }
}

impl<E> Link<E>
where
    E: edge::Adapt + Update + SendSync + ?Sized, 
{
    pub fn adapt_get(&self, deal: &mut dyn Deal) -> Result<()> {
        // read_part(&self.edge, |edge| edge.adapt_get(deal))?
        // let update = self.edge.clone() as Grc<dyn Update>;
        // let root = Root {
        //     edge: Grc::downgrade(&update),
        //     id: rand::random(),
        // };
        self.edge.adapt_get(deal, self.root.clone())
    }
    pub fn adapt_set<'a>(&'a self, deal: &'a mut dyn Deal) -> GraphFuture<Result<()>> {
        Box::pin(async move {
            // read_part(&self.edge, |edge| async move { edge.adapt_set(deal).await })?.await
            self.edge.adapt_set(deal).await
        })
    }
    pub fn passive_set(&self, deal: &mut dyn Deal) -> Result<Ring> {
        // read_part(&self.edge, |edge| edge.passive_set(deal))?
        self.edge.passive_set(deal)
    }
}

impl<T: Backed> Backed for Vec<T> {
    fn backed(&self, back: &Back) -> Self {
        self.iter().map(|link| link.backed(back)).collect()
    }
}

impl<T: Backed> Backed for Option<T> {
    fn backed(&self, back: &Back) -> Self {
        self.as_ref().map(|x| x.backed(back))
    }
}

impl<E> Link<E>
where
    E: 'static + ploy::Engage,
{
    /// Copy the link with unit type erased.  
    pub fn as_ploy(&self) -> Ploy<E::Base> {
        Ploy {
            root: self.root.clone(),
            edge: self.edge.clone(),
            path: self.path.clone(),
            rank: self.rank,
        }
    }
}

impl<E> Link<E>
where
    E: 'static + gate::Engage,
{
    /// Copy the link with unit type erased.  
    pub fn as_gate(&self) -> Gate<E::Base> {
        Gate {
            root: self.root.clone(),
            edge: self.edge.clone(),
            path: self.path.clone(),
            rank: self.rank,
        }
    }
}

impl<E> ToPloyHub for Link<E>
where
    E: 'static + ploy::Engage,
    // E::Base: PloyTag
{
    type Base = E::Base;
    fn hub(&self) -> Hub<Self::Base> {
        self.as_ploy().into()
    }
}

impl<E> ToGateHub for Link<E>
where
    E: 'static + gate::Engage,
    // E::Base: GateTag
{
    type Base = E::Base;
    fn hub(&self) -> Hub<Self::Base> {
        self.as_gate().into()
    }
}
