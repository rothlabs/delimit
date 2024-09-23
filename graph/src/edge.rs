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

impl<C> FromSnap for Edge<C>
where
    C: 'static + FromSnap + UpdateMut + AddRoot,
{
    type Unit = C::Unit;
    fn from_snap(unit: Snap<C::Unit>) -> Result<(Option<u64>, Pointer<Self>)> {
        let (rank, cusp) = C::from_snap(unit)?;
        Ok((
            rank,
            edge_pointer(Self {
                root: None,
                back: None,
                cusp,
            })
        ))
    }
}

impl<C> Solve for Edge<C>
where
    C: SolveMut + AddRoot + SendSync,
    C::Base: Payload,
{
    type Base = C::Base;
    async fn solve(&self) -> Result<Hub<Self::Base>> {
        write_part_async(&self.cusp, |mut cusp| async move {
            cusp.add_root(self.root.clone());
            cusp.solve().await
        })?
        .await
    }
    fn reckon(&self, task: Task) -> Result<Gain> {
        write_part(&self.cusp, |mut cusp| {
            cusp.add_root(self.root.clone());
            cusp.reckon(task)
        })?
    }
}

#[cfg_attr(not(feature = "oneThread"), async_trait)]
#[cfg_attr(feature = "oneThread", async_trait(?Send))]
impl<C> Adapt for Edge<C>
where
    C: AdaptMut + UpdateMut + AddRoot,
{
    fn adapt_get(&self, deal: &mut dyn Deal) -> Result<()> {
        write_part(&self.cusp, |mut cusp| {
            cusp.add_root(self.root.clone());
            cusp.adapt_get(deal)
        })?
    }
    async fn adapt_set(&self, deal: &mut dyn Deal) -> Result<()> {
        let roots = write_part(&self.cusp, |mut cusp| cusp.adapt_set(deal))??;
        for root in roots.iter() {
            root.react().await?;
        }
        Ok(())
    }
}

#[cfg_attr(not(feature = "oneThread"), async_trait)]
#[cfg_attr(feature = "oneThread", async_trait(?Send))]
impl<C> Based for Edge<C>
where
    C: 'static + SolveMut + UpdateMut + AdaptMut + AddRoot + Debug,
{
    type Base = C::Base;
    
    async fn solve(&self) -> Result<Hub<Self::Base>> {
        write_part_async(&self.cusp, |mut cusp| async move {
            cusp.add_root(self.root.clone());
            cusp.solve().await
        })?.await
    }
    fn backed(&self, back: &Back) -> PloyEdge<Self::Base> {
        edge_pointer(Self {
            root: None, 
            back: Some(back.clone()),
            cusp: self.cusp.clone(),
        })
    }
    fn reckon(&self, task: Task) -> Result<Gain> {
        write_part(&self.cusp, |mut cusp| {
            cusp.add_root(self.root.clone());
            cusp.reckon(task)
        })?
    }
}

impl<C> BackedMid for Edge<C> 
where
    C: 'static + ReactMut + AddRoot + SendSync,
{
    fn backed(&self, back: &Back) -> Pointer<Self> {
        edge_pointer(Self {
            root: None,
            back: Some(back.clone()),
            cusp: self.cusp.clone(),
        })
    }
}

#[cfg_attr(not(feature = "oneThread"), async_trait)]
#[cfg_attr(feature = "oneThread", async_trait(?Send))]
impl<C, T> WriteBase<T> for Edge<C>
where
    C: WriteBaseOut<T> + SendSync,
{
    async fn write<O: SendSync, F: FnOnce(&mut T) -> O + SendSync>(&self, write: F) -> Result<O> {
        let Post { roots, out } =
            write_part(&self.cusp, |mut cusp| cusp.write_base_out(write))??;
        for root in roots.iter() {
            root.react().await?;
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
        let Post { roots, out } =
            write_part(&self.cusp, |mut cusp| cusp.write_unit_out(write))??;
        for root in roots.iter() {
            root.react().await?;
        }
        Ok(out)
    }
}

impl<N> Read for Edge<N>
where
    N: ToItem + AddRoot,
{
    type Item = N::Item;
    fn read<T, F: FnOnce(&Self::Item) -> T>(&self, read: F) -> Result<T> {
        write_part(&self.cusp, |mut cusp| {
            cusp.add_root(self.root.clone());
            read(cusp.item())
        })
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
    N: ReactMut + AddRoot + SendSync,
{
    async fn react(&self) -> Result<()> {
        write_part_async(&self.cusp, |mut cusp| async move {
            cusp.add_root(self.root.clone());
            cusp.react().await
        })?
        .await
    }
}