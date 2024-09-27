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
pub struct Edge<C> {
    cusp: Pointer<C>,
    root: Option<Root>,
    back: Option<Back>,
}

impl<C> FromBase for Edge<C>
where
    C: 'static + FromBase + ReactMut + AddRoot + SendSync,
{
    type Base = C::Base;
    fn from_base(base: C::Base) -> Pointer<Self> {
        edge_pointer(Self {
            root: None,
            back: None,
            cusp: C::from_base(base),
        })
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
            }),
        ))
    }
}

impl<C> Solve for Edge<C>
where
    C: SolveMut + AddRoot + SendSync,
{
    type Base = C::Base;
    async fn solve(&self) -> Result<Hub<Self::Base>> {
        write_part(&self.cusp, |mut cusp| async move {
            cusp.add_root(&self.root);
            cusp.solve().await
        })?
        .await
    }
    fn reckon(&self, task: Task) -> Result<Gain> {
        write_part(&self.cusp, |mut cusp| {
            cusp.add_root(&self.root);
            cusp.reckon(task)
        })?
    }
}

impl<C> Adapt for Edge<C>
where
    C: AdaptMut + UpdateMut + AddRoot,
{
    fn adapt_get(&self, deal: &mut dyn Deal) -> Result<()> {
        write_part(&self.cusp, |mut cusp| {
            cusp.add_root(&self.root);
            cusp.adapt_get(deal)
        })?
    }
    fn adapt_set<'a>(&'a self, deal: &'a mut dyn Deal) -> GraphFuture<Result<()>> {
        Box::pin(async move {
            let ring = write_part(&self.cusp, |mut cusp| cusp.adapt_set(deal))??;
            ring.react().await
        })
    }
    fn transient_set(&self, deal: &mut dyn Deal) -> Result<Ring> {
        write_part(&self.cusp, |mut cusp| cusp.adapt_set(deal))?
    }
}

impl<C> Based for Edge<C>
where
    C: 'static + SolveMut + UpdateMut + AdaptMut + AddRoot + Debug,
{
    type Base = C::Base;
    fn solve(&self) -> GraphFuture<Result<Hub<Self::Base>>> {
        Box::pin(async move {
            write_part(&self.cusp, |mut cusp| async move {
                cusp.add_root(&self.root);
                cusp.solve().await
            })?
            .await
        })
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
            cusp.add_root(&self.root);
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

impl<C> WriteBase for Edge<C>
where
    C: WriteBaseOut + SendSync,
{
    type Base = C::Base;
    async fn write<O, F>(&self, write: F) -> Result<O>
    where
        F: FnOnce(&mut C::Base) -> O,
    {
        let (ring, out) = write_part(&self.cusp, |mut cusp| cusp.write_base_out(write))??;
        ring.react().await?;
        Ok(out)
    }
}

impl<C> WriteUnit for Edge<C>
where
    C: WriteUnitOut + UpdateMut,
{
    type Unit = C::Unit;
    async fn write<O, F>(&self, write: F) -> Result<O>
    where
        F: FnOnce(&mut Pack<C::Unit>) -> O,
    {
        let (ring, out) = write_part(&self.cusp, |mut cusp| cusp.write_unit_out(write))??;
        ring.react().await?;
        Ok(out)
    }
}

impl<C> Read for Edge<C>
where
    C: ToItem + AddRoot,
{
    type Item = C::Item;
    fn read<T, F>(&self, read: F) -> Result<T>
    where
        F: FnOnce(&C::Item) -> T,
    {
        write_part(&self.cusp, |mut cusp| {
            cusp.add_root(&self.root);
            read(cusp.item())
        })
    }
}

impl<C> Rebut for Edge<C> {
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

impl<C> React for Edge<C>
where
    C: ReactMut + AddRoot + SendSync,
{
    fn react(&self) -> GraphFuture<Result<()>> {
        Box::pin(async move {
            write_part(&self.cusp, |mut cusp| async move {
                cusp.add_root(&self.root);
                cusp.react().await
            })?
            .await
        })
    }
}

#[cfg(not(feature = "oneThread"))]
fn edge_pointer<T>(edge: T) -> Arc<RwLock<T>>
where
    T: 'static + Update + SetRoot,
{
    let edge = Arc::new(RwLock::new(edge));
    let update = edge.clone() as Arc<RwLock<dyn Update>>;
    let root = Root {
        edge: Arc::downgrade(&update),
        id: rand::random(),
    };
    edge.write().set_root(root);
    edge
}

#[cfg(feature = "oneThread")]
fn edge_pointer<T>(edge: T) -> Rc<RefCell<T>>
where
    T: 'static + Update + SetRoot,
{
    let edge = Rc::new(RefCell::new(edge));
    let update = edge.clone() as Rc<RefCell<dyn Update>>;
    let root = Root {
        edge: Rc::downgrade(&update),
        id: rand::random(),
    };
    edge.borrow_mut().set_root(root);
    edge
}
