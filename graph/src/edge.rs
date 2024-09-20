use async_trait::async_trait;

use super::*;
#[cfg(not(feature = "oneThread"))]
use parking_lot::RwLock;
#[cfg(not(feature = "oneThread"))]
use std::sync::Arc;
#[cfg(feature = "oneThread")]
use std::{cell::RefCell, rc::Rc};

/// Edge to a tray.
pub type Leaf<T> = Edge<cusp::Leaf<T>>;

/// Edge to a unit that grants a tray.
pub type Node<U> = Edge<cusp::Node<U>>;

/// The forward bridge between hubes.
#[derive(Debug)]
pub struct Edge<N> {
    root: Option<Root>,
    back: Option<Back>,
    #[cfg(not(feature = "oneThread"))]
    cusp: Arc<RwLock<N>>,
    #[cfg(feature = "oneThread")]
    cusp: Rc<RefCell<N>>,
}

impl<C> Default for Edge<C>
where
    C: Default,
{
    fn default() -> Self {
        Self {
            root: None,
            back: None,
            #[cfg(not(feature = "oneThread"))]
            cusp: Arc::new(RwLock::new(C::default())),
            #[cfg(feature = "oneThread")]
            cusp: Rc::new(RefCell::new(C::default())),
        }
    }
}

impl<N> FromItem for Edge<N>
where
    N: FromItem,
{
    type Item = N::Item;
    fn new(unit: Self::Item) -> Self {
        let cusp = N::new(unit);
        Self {
            root: None,
            back: None,
            #[cfg(not(feature = "oneThread"))]
            cusp: Arc::new(RwLock::new(cusp)),
            #[cfg(feature = "oneThread")]
            cusp: Rc::new(RefCell::new(cusp)),
        }
    }
}

impl<C> SetRoot for Edge<C> {
    fn set_root(&mut self, root: Root) {
        self.root = Some(root);
    }
}

impl<C> Make for Edge<C>
where
    C: 'static + Default + InitMut + UpdateMut + AddRootMut,
{
    type Unit = C::Unit;
    #[cfg(not(feature = "oneThread"))]
    fn make<F: FnOnce(&Back) -> Result<Self::Unit>>(make: F) -> Result<(Pointer<Self>, Option<u64>)> {
        let cusp = C::default();
        let id = cusp.id();
        let cusp = Arc::new(RwLock::new(cusp));
        let update = cusp.clone() as Arc<RwLock<dyn UpdateMut>>;
        let back = Back::new(Arc::downgrade(&update), id);
        let rank = cusp.write().init(make, &back)?;
        Ok((
            edge_pointer(Self {
                root: None,
                back: None,
                cusp,
            }),
            rank,
        ))
    }
    #[cfg(feature = "oneThread")]
    fn init<F: FnOnce(&Back) -> Result<Self::Unit>>(make: F) -> Result<(Pointer<Self>, Option<u64>)> {
        let cusp = C::default();
        let id = cusp.id();
        let cusp = Rc::new(RefCell::new(cusp));
        let update = cusp.clone() as Rc<RefCell<dyn UpdateMut>>;
        let back = Back::new(Rc::downgrade(&update), id);
        let rank = write_part(&cusp, |mut cusp| cusp.init(make, &back))??;
        Ok((
            edge_pointer(Self {
                root: None,
                back: None,
                cusp,
            }),
            rank,
        ))
    }
}

impl<N> FromSnap for Edge<N>
where
    N: 'static + Default + WithSnap + UpdateMut + AddRootMut,
{
    type Unit = N::Unit;
    #[cfg(not(feature = "oneThread"))]
    fn from_snap(snap: Snap<Self::Unit>) -> (Pointer<Self>, Option<u64>) {
        let cusp = N::default();
        let id = cusp.id();
        let cusp = Arc::new(RwLock::new(cusp));
        let update = cusp.clone() as Arc<RwLock<dyn UpdateMut>>;
        let back = Back::new(Arc::downgrade(&update), id);
        let rank = cusp.write().with_snap(snap, &back);
        (
            edge_pointer(Self {
                cusp,
                back: None,
                root: None,
            }),
            rank,
        )
    }
    #[cfg(feature = "oneThread")]
    fn from_snap(snap: Snap<Self::Unit>) -> (Pointer<Self>, Option<u64>) {
        let cusp = N::default();
        let id = cusp.id();
        let cusp = Rc::new(RefCell::new(cusp));
        let update = cusp.clone() as Rc<RefCell<dyn UpdateMut>>;
        let back = Back::new(Rc::downgrade(&update), id);
        let rank =
            write_part(&cusp, |mut cusp| cusp.with_snap(snap, &back)).expect(IMMEDIATE_ACCESS);
        (
            edge_pointer(Self {
                cusp,
                back: None,
                root: None,
            }),
            rank,
        )
    }
}

// #[cfg_attr(not(feature = "oneThread"), async_trait)]
// #[cfg_attr(feature = "oneThread", async_trait(?Send))]
impl<C> Solve for Edge<C>
where
    C: 'static + SolveMut + AddRootMut + SendSync,
    C::Base: Payload,
{
    type Base = C::Base;
    async fn solve(&self) -> Result<Hub<Self::Base>> {
        write_part_async(&self.cusp, |mut cusp| async move {
            let out = cusp.solve().await;
            if let Some(root) = self.root.clone() {
                cusp.add_root(root);
            }
            out
        })?
        .await
    }
}

impl<C> Reckon for Edge<C>
where
    C: ReckonMut, // + AddRoot,
{
    fn reckon(&self, task: Task) -> Result<Gain> {
        // should not need to add root
        write_part(&self.cusp, |mut cusp| {
            cusp.reckon(task)
            // if let Some(root) = self.root.clone() {
            //     cusp.add_root(root);
            // }
            // out
        })?
    }
}

impl<C> AdaptGet for Edge<C>
where
    C: 'static + AdaptOut + UpdateMut,
{
    fn adapt_get(&self, deal: &mut dyn Deal) -> Result<()> {
        write_part(&self.cusp, |mut cusp| cusp.adapt(deal))??;
        Ok(())
    }
}

#[cfg_attr(not(feature = "oneThread"), async_trait)]
#[cfg_attr(feature = "oneThread", async_trait(?Send))]
impl<C> AdaptSet for Edge<C>
where
    C: 'static + AdaptOut + UpdateMut,
{
    async fn adapt_set(&self, deal: &mut dyn Deal) -> Result<()> {
        let (roots, id) = write_part(&self.cusp, |mut cusp| cusp.adapt(deal))??;
        if deal.wrote() {
            for root in roots.iter() {
                root.react(&id).await?;
            }
        }
        Ok(())
    }
}

#[cfg_attr(not(feature = "oneThread"), async_trait)]
#[cfg_attr(feature = "oneThread", async_trait(?Send))]
impl<C> Based for Edge<C>
where
    C: 'static + SolveMut + UpdateMut + ReckonMut + SendSync,
    C: AdaptOut + AddRootMut + Debug,
    C::Base: Payload,
{
    type Base = C::Base;
    async fn solve(&self) -> Result<Hub<Self::Base>> {
        write_part_async(&self.cusp, |mut cusp| async move {
            let out = cusp.solve().await;
            if let Some(root) = self.root.clone() {
                cusp.add_root(root);
            }
            out
        })?
        .await
    }
    fn backed(&self, back: &Back) -> PloyEdge<Self::Base> {
        edge_pointer(Self {
            root: None, 
            back: Some(back.clone()),
            cusp: self.cusp.clone(),
        })
    }
}

impl<C> BackedMid for Edge<C> 
where
    C: 'static + ReactMut + AddRootMut + SendSync,
{
    fn backed(&self, back: &Back) -> Pointer<Self> {
        let edge = Self {
            root: self.root.clone(),
            back: Some(back.clone()),
            cusp: self.cusp.clone(),
        };
        let wow = edge_pointer(edge);
        wow
    }
}

#[cfg_attr(not(feature = "oneThread"), async_trait)]
#[cfg_attr(feature = "oneThread", async_trait(?Send))]
impl<C, T> WriteBase<T> for Edge<C>
where
    C: WriteBaseOut<T> + SendSync,
{
    async fn write<O: SendSync, F: FnOnce(&mut T) -> O + SendSync>(&self, write: F) -> Result<O> {
        let Post { roots, id, out } =
            write_part(&self.cusp, |mut cusp| cusp.write_base_out(write))??;
        for root in roots.iter() {
            root.react(&id).await?;
        }
        Ok(out)
    }
}

#[cfg_attr(not(feature = "oneThread"), async_trait)]
#[cfg_attr(feature = "oneThread", async_trait(?Send))]
impl<N> WriteUnit for Edge<N>
where
    N: 'static + WriteUnitOut + UpdateMut,
{
    type Unit = N::Unit;
    async fn write<T: SendSync, F: FnOnce(&mut Pack<Self::Unit>) -> T + SendSync>(
        &self,
        write: F,
    ) -> Result<T> {
        let Post { roots, id, out } =
            write_part(&self.cusp, |mut cusp| cusp.write_unit_out(write))??;
        for root in roots.iter() {
            root.react(&id).await?;
        }
        Ok(out)
    }
}

impl<N> Read for Edge<N>
where
    N: ToItem + AddRootMut,
{
    type Item = N::Item;
    fn read<T, F: FnOnce(&Self::Item) -> T>(&self, read: F) -> Result<T> {
        write_part(&self.cusp, |mut cusp| {
            let out = read(cusp.item());
            if let Some(root) = self.root.clone() {
                cusp.add_root(root);
            }
            out
        })
    }
}

impl<N> AddRoot for Edge<N>
where
    N: AddRootMut,
{
    fn add_root(&self, root: Root) -> Result<()> {
        write_part(&self.cusp, |mut cusp| cusp.add_root(root))
    }
}

impl<N> Rebut for Edge<N> {
    fn rebut(&self) -> Result<Ring> {
        if let Some(back) = &self.back {
            back.rebut()
        } else {
            Ok(Ring::new())
        }
    }
    fn clear_roots(&self) -> Result<()> {
        if let Some(back) = &self.back {
            back.clear()
        } else {
            Ok(())
        }
    }
}

#[cfg_attr(not(feature = "oneThread"), async_trait)]
#[cfg_attr(feature = "oneThread", async_trait(?Send))]
impl<N> React for Edge<N>
where
    N: ReactMut + AddRootMut + SendSync,
{
    async fn react(&self, id: &Id) -> Result<()> {
        write_part_async(&self.cusp, |mut cusp| async move {
            let out = cusp.react(id).await;
            if let Some(root) = self.root.clone() {
                cusp.add_root(root);
            }
            out
        })?
        .await
    }
}

// impl<N> ToId for Edge<N> {
//     fn id(&self) -> Id {
//         if let Some(back) = &self.back {
//             back.id
//         } else {
//             0
//         }
//     }
// }
