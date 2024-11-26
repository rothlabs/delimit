use super::*;

pub(crate) mod gate;
pub(crate) mod ploy;

/// Edge to a tray.
pub type Leaf<T> = Edge<cusp::Leaf<T>>;

/// Edge to a unit that grants a tray.
pub type Node<U> = Edge<cusp::Node<U>>;

/// The forward bridge between hubes.
#[derive(Default, Debug)]
pub struct Edge<C> {
    cusp: Pointer<C>,
    // root: Option<Root>,
    back: Option<Back>,
}

// impl<C> Edge<C> 
// where
//     C: 'static + FromBase + ReactMut + AddRoot + SendSync,
// {
//     pub fn from_base2(base: C::Base) -> Grc<Self> {
//         Self {
//             cusp: C::from_base(base),
//             back: None,
//         }.into()
//     }
// }

// fn root_edge<E: 'static + Update>(edge: &Grc<E>) -> Root {
//     let update = edge.clone() as Grc<dyn Update>;
//     Root {
//         edge: Grc::downgrade(&update),
//         id: rand::random(),
//     }
// }

pub trait FromBase {
    type Base;
    fn from_base(base: Self::Base) -> (Grc<Self>, Root);
}

impl<C> FromBase for Edge<C>
where
    C: 'static + cusp::FromBase + ReactMut + AddRoot + SendSync,
{
    type Base = C::Base;
    fn from_base(base: C::Base) -> (Grc<Self>, Root) {
        let edge = Grc::new(Self {
            cusp: C::from_base(base),
            back: None,
        });
        // let root = root_edge(&edge);
        (edge.clone(), Root::new(edge))
    }
}

// impl<C> SetRoot for Edge<C> {
//     fn set_root(&mut self, root: Root) {
//         self.root = Some(root);
//     }
// }

pub trait FromSnap {
    type Unit;
    fn from_snap(unit: Snap<Self::Unit>) -> Result<(Option<u16>, Grc<Self>, Root)>;
}

impl<C> FromSnap for Edge<C>
where
    C: 'static + cusp::FromSnap + UpdateMut + AddRoot,
{
    type Unit = C::Unit;
    fn from_snap(unit: Snap<C::Unit>) -> Result<(Option<u16>, Grc<Self>, Root)> {
        let (rank, cusp) = C::from_snap(unit)?;
        let edge = Grc::new(Self {
            back: None,
            cusp,
        });
        Ok((
            rank,
            edge.clone(),
            Root::new(edge)
        ))
    }
}

pub trait Solve {
    type Base: 'static + SendSync;
    /// Solve a task.
    /// The node will run computations or return existing results.
    fn solve(&self, root: Root) -> impl Future<Output = node::Result<Self::Base>> + IsSend;
}

impl<C> Solve for Edge<C>
where
    C: cusp::Solve + AddRoot + SendSync,
{
    type Base = C::Base;
    async fn solve(&self, root: Root) -> node::Result<Self::Base> {
        Ok(write_part(&self.cusp, |mut cusp| async move {
            cusp.add_root(root);
            cusp.solve().await
        })?
        .await?)
    }
}

pub trait Adapt {
    /// For graph internals to handle alter calls
    fn adapt_get(&self, deal: &mut dyn Deal, root: Root) -> Result<()>;
    /// For graph internals to handle alter calls
    fn adapt_set<'a>(&'a self, deal: &'a mut dyn Deal) -> GraphFuture<Result<()>>;
    fn passive_set(&self, deal: &mut dyn Deal) -> Result<Ring>;
}

impl<C> Adapt for Edge<C>
where
    C: cusp::Adapt + UpdateMut + AddRoot,
{
    fn adapt_get(&self, deal: &mut dyn Deal, root: Root) -> Result<()> {
        write_part(&self.cusp, |mut cusp| {
            cusp.add_root(root);
            cusp.adapt_get(deal)
        })?
    }
    fn adapt_set<'a>(&'a self, deal: &'a mut dyn Deal) -> GraphFuture<Result<()>> {
        Box::pin(async move {
            let ring = write_part(&self.cusp, |mut cusp| cusp.adapt_set(deal))??;
            ring.react().await
        })
    }
    fn passive_set(&self, deal: &mut dyn Deal) -> Result<Ring> {
        write_part(&self.cusp, |mut cusp| cusp.adapt_set(deal))?
    }
}

impl<C> ploy::Solve for Edge<C>
where
    C: 'static + cusp::Solve + UpdateMut + cusp::Adapt + AddRoot + ReckonMut + Debug,
{
    type Base = C::Base;
    fn solve(&self, root: Root) -> GraphFuture<Result<Hub<Self::Base>>> {
        Box::pin(async move {
            write_part(&self.cusp, |mut cusp| async move {
                cusp.add_root(root);
                cusp.solve().await
            })?
            .await
        })
    }
    fn backed(&self, back: &Back) -> (ploy::Edge<Self::Base>, Root) {
        let edge = Grc::new(Self {
            back: Some(back.clone()),
            cusp: self.cusp.clone(),
        });
        (edge.clone(), Root::new(edge))
    }
}

impl<C> gate::Solve for Edge<C>
where
    C: 'static + cusp::Solve + UpdateMut + cusp::Adapt + AddRoot + Debug + GateTag,
{
    type Base = C::Base;
    fn solve(&self, root: Root) -> GraphFuture<Result<Hub<Self::Base>>> {
        Box::pin(async move {
            write_part(&self.cusp, |mut cusp| async move {
                cusp.add_root(root);
                cusp.solve().await
            })?
            .await
        })
    }
    fn backed(&self, back: &Back) -> (gate::Edge<Self::Base>, Root){
        let edge = Grc::new(Self {
            back: Some(back.clone()),
            cusp: self.cusp.clone(),
        });
        (edge.clone(), Root::new(edge))
    }
}

impl<C> Reckon for Edge<C>
where
    C: ReckonMut,
{
    fn get_imports(&self) -> Result<Vec<Import>> {
        read_part(&self.cusp, |cusp| cusp.get_imports())?
    }
    fn get_hash(&self) -> Result<u64> {
        write_part(&self.cusp, |mut cusp| cusp.get_hash())?
    }
    fn get_serial(&self) -> Result<String> {
        write_part(&self.cusp, |mut cusp| cusp.get_serial())?
    }
}

pub trait BackedMid {
    // type Cusp;
    /// Make a copy of the link that includes the provided cusp `&Back` on the edge.
    /// Must be called to include `&Back` in the rebut phase.
    fn backed(&self, back: &Back) -> (Grc<Self>, Root);
}

impl<C> BackedMid for Edge<C>
where
    C: 'static + ReactMut + AddRoot + SendSync,
{
    fn backed(&self, back: &Back) -> (Grc<Self>, Root) {
        // Self {
        //     back: Some(back.clone()),
        //     cusp: self.cusp.clone(),
        // }.into()
        let edge = Grc::new(Self {
            back: Some(back.clone()),
            cusp: self.cusp.clone(),
        });
        (edge.clone(), Root::new(edge))
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
    fn write_passive<O, F>(&self, write: F) -> Result<O>
    where
        F: FnOnce(&mut Self::Base) -> O,
    {
        let out = write_part(&self.cusp, |mut cusp| cusp.write_base_out_passive(write))??;
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

pub trait Read {
    type Item;
    /// Read the Unit or Transmit of the graph part.
    fn read<T, F>(&self, reader: F, root: Root) -> Result<T>
    where
        F: FnOnce(&Self::Item) -> T;
}

impl<C> Read for Edge<C>
where
    C: ToItem + AddRoot,
{
    type Item = C::Item;
    fn read<T, F>(&self, read: F, root: Root) -> Result<T>
    where
        F: FnOnce(&C::Item) -> T,
    {
        write_part(&self.cusp, |mut cusp| {
            cusp.add_root(root);
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

pub trait React {
    /// Cause the unit to react. Call only on graph roots returned from the rebut phase.
    fn react(&self, root: Root) -> GraphFuture<Result<()>>;
}

impl<C> React for Edge<C>
where
    C: ReactMut + AddRoot + SendSync,
{
    fn react(&self, root: Root) -> GraphFuture<Result<()>> {
        Box::pin(async move {
            write_part(&self.cusp, |mut cusp| async move {
                cusp.add_root(root);
                cusp.react().await
            })?
            .await
        })
    }
}

// #[cfg(not(feature = "oneThread"))]
// fn edge_pointer<T>(edge: T) -> Arc<RwLock<T>>
// where
//     T: 'static + Update// + SetRoot,
// {
//     let edge = Arc::new(RwLock::new(edge));
//     let update = edge.clone() as Arc<RwLock<dyn Update>>;
//     let root = Root {
//         edge: Arc::downgrade(&update),
//         id: rand::random(),
//     };
//     // edge.write().set_root(root);
//     edge
// }

// #[cfg(feature = "oneThread")]
// fn edge_pointer<T>(edge: T) -> Rc<RefCell<T>>
// where
//     T: 'static + Update// + SetRoot,
// {
//     let edge = Rc::new(RefCell::new(edge));
//     let update = edge.clone() as Rc<RefCell<dyn Update>>;
//     let root = Root {
//         edge: Rc::downgrade(&update),
//         id: rand::random(),
//     };
//     // edge.borrow_mut().set_root(root);
//     edge
// }
