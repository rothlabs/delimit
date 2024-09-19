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

impl<E> Hash for Link<E>
where
    Self: Solve + Reckon,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Some(path) = &self.path {
            path.hash(state)
        } else if let Ok(Gain::U64(hash)) = self.reckon(Task::Hash) {
            hash.hash(state)
        }
    }
}

impl<E> Serialize for Link<E>
where
    Self: Solve + Reckon
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

#[cfg(not(feature = "oneThread"))]
fn edge_pointers<T>(edge: T) -> (Arc<RwLock<T>>, Root) 
where 
    T: 'static + Update,
{
    let edge = Arc::new(RwLock::new(edge));
    let update = edge.clone() as Arc<RwLock<dyn Update>>;
    (edge, Root{ edge: Arc::downgrade(&update), id: rand::random() })
}

#[cfg(feature = "oneThread")]
fn edge_pointers<T>(edge: T) -> (Rc<RefCell<T>>, Root) 
where 
    T: 'static + Update,
{
    let edge = Rc::new(RefCell::new(edge));
    let update = edge.clone() as Rc<RefCell<dyn Update>>;
    (edge, Root{ edge: Rc::downgrade(&update), id: rand::random() })
}

impl<E> Link<E>
where
    E: 'static + FromItem + SetRoot + Update,
{
    pub fn new(base: E::Item) -> Self {
        let (edge, root) = edge_pointers(E::new(base));
        write_part(&edge, |mut edge| edge.set_root(root)).expect(IMMEDIATE_ACCESS);
        Self {
            path: None,
            rank: None,
            edge,
        }
    }
}

impl<E> Link<E>
where
    E: 'static + Default + InitEdge + Update,
{
    pub fn make<F: FnOnce(&Back) -> Result<E::Unit>>(make: F) -> Result<Self> {
        let (edge, root) = edge_pointers(E::default());
        let rank = write_part(&edge, |mut edge| edge.init(make, root))??;
        Ok(Self {
            path: None,
            rank,
            edge,
        })
    }
}

impl<E> Link<E>
where
    E: 'static + Default + InitEdge + Engage,
{
    pub fn make_ploy<F: FnOnce(&Back) -> Result<E::Unit>>(make: F) -> Result<Ploy<E::Base>> {
        // let (edge, rank) = E::make(make)?;
        // let (edge, root) = edge_pointers(Box::new(E::default()));
        // let rank = write_part(&edge, |mut edge| edge.init(make, root))??;
        let edge = Box::new(E::default()) as Box<dyn Engage<Base = E::Base>>;
        let (edge, root) = edge_pointers(edge);
        let rank = write_part(&edge, |mut edge| edge.set_root(root))?;
        Ok(Link {
            path: None,
            rank,
            edge
            // #[cfg(not(feature = "oneThread"))]
            // edge: Arc::new(RwLock::new(
            //     Box::new(edge) as Box<dyn Engage<Base = E::Base>>
            // )),
            // #[cfg(feature = "oneThread")]
            // edge: Rc::new(RefCell::new(
            //     Box::new(edge) as Box<dyn Engage<Base = E::Base>>
            // )),
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

// impl<E> Link<E>
// where
//     E: ToPloy + Clone + Update + SetRoot,
// {
//     /// Copy the link with unit type erased.  
//     pub fn ploy(&self) -> Result<Ploy<E::Base>> {
//         // let (edge, root) = edge_pointers(edge.ploy());
//         // write_part(&edge, |mut edge| edge.root(root))?;
//         read_part(&self.edge, |edge| {
//             let (edge, root) = edge_pointers((*edge).clone());
//             write_part(&edge, |mut edge| edge.root(root))?;
//             Ok(Ploy {
//                 edge,
//                 path: self.path.clone(),
//                 rank: self.rank,
//             })
//         })?
//     }
// }

impl<E> Link<E>
where
    E: 'static + FromSnap + Engage,
{
    // TODO: add weak self to edge!!!
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
    E: 'static + BackedMid + SetRoot + Update,
{
    fn backed(&self, back: &Back) -> Result<Self> {
        read_part(&self.edge, |edge| {
            let (edge, root) = edge_pointers(edge.backed(back));
            write_part(&edge, |mut edge| edge.set_root(root))?;
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
        read_part(&self.edge, |edge| {
            let edge = edge.backed(back);
            #[cfg(not(feature = "oneThread"))]
            let (edge, root) = {
                let update = edge.clone() as Arc<RwLock<dyn Update>>;
                (edge, Root{ edge: Arc::downgrade(&update), id: rand::random() })
            };
            #[cfg(feature = "oneThread")]
            let (edge, root) = {
                let update = edge.clone() as Rc<RefCell<dyn Update>>;
                (edge, Root{ edge: Rc::downgrade(&update), id: rand::random() })
            };
            write_part(&edge, |mut edge| edge.set_root(root))?;
            Ok(Self {
                edge,//: edge.backed(back),
                path: self.path.clone(),
                rank: self.rank,
            })
        })?
    }
}

impl<E> Link<E>
where
    E: 'static + Read + Update + AddRoot,
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
    E: 'static + Solve + AddRoot + Update,
    E::Base: Payload,
{
    type Base = E::Base;
    async fn solve(&self) -> Result<Hub<Self::Base>> {
        read_part_async(&self.edge, |edge| async move {edge.solve().await})?.await
    }
}

impl<E> Reckon for Link<E>
where
    E: 'static + Reckon + AddRoot + Update,
{
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
        read_part_async(&self.edge, |edge| async move {edge.solve().await})?.await
    }
}

impl<E> AdaptGet for Link<E>
where
    E: 'static + AdaptGet + AddRoot + Update,
{
    fn adapt_get(&self, deal: &mut dyn Deal) -> Result<()> {
        read_part(&self.edge, |edge| {
            let out = edge.adapt_get(deal);
            // if deal.read() {
            //     edge.add_root(self.as_root(edge.id()))?;
            // }
            // } else {
            //     return Err(anyhow!("Deal did not report reading in AdaptGet"))?;
            // }
            // if deal.wrote() {
            //     return Err(anyhow!("Deal should not write in AdaptGet"))?;
            // }
            out
        })?
    }
}

#[cfg_attr(not(feature = "oneThread"), async_trait)]
#[cfg_attr(feature = "oneThread", async_trait(?Send))]
impl<E> AdaptSet for Link<E>
where
    E: 'static + AdaptSet + AddRoot + Update,
{
    async fn adapt_set(&self, deal: &mut dyn Deal) -> Result<()> {
        read_part_async(&self.edge, |edge| async move {
            let result = edge.adapt_set(deal).await;
            // if deal.read() {
            //     return Err(anyhow!("Deal should not read in AdaptSet"))?;
            // }
            // if !deal.wrote() {
            //     return Err(anyhow!("Deal did not report writing in AdaptSet"))?;
            // }
            result
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


// impl<E> Link<E>
// where
//     E: 'static + Update,
// {
//     #[cfg(not(feature = "oneThread"))]
//     pub fn as_root(&self, id: Id) -> Root {
//         let edge = self.edge.clone() as Arc<RwLock<dyn Update>>;
//         Root {
//             edge: Arc::downgrade(&edge),
//             id,
//         }
//     }
//     #[cfg(feature = "oneThread")]
//     pub fn as_root(&self, id: Id) -> Root {
//         let edge = self.edge.clone() as Rc<RefCell<dyn Update>>;
//         Root {
//             edge: Rc::downgrade(&edge),
//             id,
//         }
//     }
// }