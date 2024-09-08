use super::*;
#[cfg(not(feature = "oneThread"))]
use std::sync::{Arc, RwLock};
#[cfg(feature = "oneThread")]
use std::{cell::RefCell, rc::Rc};

/// Edge to a tray.
pub type Leaf<T> = Edge<cusp::Leaf<T>>;

/// Edge to a unit that grants a tray.
pub type Node<U> = Edge<cusp::Node<U>>;

/// The forward bridge between hubes.
#[derive(Debug)]
pub struct Edge<N> {
    pub back: Option<Back>,
    #[cfg(not(feature = "oneThread"))]
    pub cusp: Arc<RwLock<N>>,
    #[cfg(feature = "oneThread")]
    pub cusp: Rc<RefCell<N>>,
}

impl<N> ToId for Edge<N> {
    fn id(&self) -> Id {
        if let Some(back) = &self.back {
            back.id
        } else {
            0
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
            back: None,
            #[cfg(not(feature = "oneThread"))]
            cusp: Arc::new(RwLock::new(cusp)),
            #[cfg(feature = "oneThread")]
            cusp: Rc::new(RefCell::new(cusp)),
        }
    }
}

impl<C> Make for Edge<C>
where
    C: 'static + Default + MakeMid + UpdateMut, 
{
    type Unit = C::Unit;
    #[cfg(not(feature = "oneThread"))]
    fn make<F: FnOnce(&Back) -> Result<Self::Unit>>(make: F) -> Result<(Self, Option<u64>)> {
        let cusp = C::default();
        let id = cusp.id();
        let cusp = Arc::new(RwLock::new(cusp));
        let update = cusp.clone() as Arc<RwLock<dyn UpdateMut>>;
        let back = Back::new(Arc::downgrade(&update), id);
        let rank = write_part(&cusp, |mut cusp| cusp.make(make, &back))??;
        Ok((Self { cusp, back: None }, rank))
    }
    #[cfg(feature = "oneThread")]
    fn make<F: FnOnce(&Back) -> Result<Self::Unit>>(make: F) -> Result<(Self, Option<u64>)> {
        let cusp = C::default();
        let id = cusp.id();
        let cusp = Rc::new(RefCell::new(cusp));
        let update = cusp.clone() as Rc<RefCell<dyn UpdateMut>>;
        let back = Back::new(Rc::downgrade(&update), id);
        let rank = write_part(&cusp, |mut cusp| cusp.make(make, &back))??;
        Ok((Self { cusp, back: None }, rank))
    }
}

impl<U> ToPloy for Node<U>
where
    U: 'static + Solve + Adapt + Debug + SendSync,
    U::Base: Payload,
{
    type Base = U::Base;
    #[cfg(not(feature = "oneThread"))]
    fn ploy(&self) -> PloyPointer<U::Base> {
        Arc::new(RwLock::new(Box::new(Self {
            back: self.back.clone(),
            cusp: self.cusp.clone(),
        })))
    }
    #[cfg(feature = "oneThread")]
    fn ploy(&self) -> PloyPointer<U::Base> {
        Rc::new(RefCell::new(Box::new(Self {
            back: self.back.clone(),
            cusp: self.cusp.clone(),
        })))
    }
}

impl<N> FromSnap for Edge<N>
where
    N: 'static + Default + WithSnap + UpdateMut,
{
    type Unit = N::Unit;
    #[cfg(not(feature = "oneThread"))]
    fn from_snap(snap: Snap<Self::Unit>) -> (Self, Option<u64>) {
        let cusp = N::default();
        let id = cusp.id();
        let cusp = Arc::new(RwLock::new(cusp));
        let update = cusp.clone() as Arc<RwLock<dyn UpdateMut>>;
        let back = Back::new(Arc::downgrade(&update), id);
        let rank =
            write_part(&cusp, |mut cusp| cusp.with_snap(snap, &back)).expect(IMMEDIATE_ACCESS);
        (Self { cusp, back: None }, rank)
    }
    #[cfg(feature = "oneThread")]
    fn from_snap(snap: Snap<Self::Unit>) -> (Self, Option<u64>) {
        let cusp = N::default();
        let id = cusp.id();
        let cusp = Rc::new(RefCell::new(cusp));
        let update = cusp.clone() as Rc<RefCell<dyn UpdateMut>>;
        let back = Back::new(Rc::downgrade(&update), id);
        let rank =
            write_part(&cusp, |mut cusp| cusp.with_snap(snap, &back)).expect(IMMEDIATE_ACCESS);
        (Self { cusp, back: None }, rank)
    }
}

impl<C> Solve for Edge<C>
where
    C: 'static + SolveMut + UpdateMut,
    C::Base: Payload,
{
    type Base = C::Base;
    fn solve(&self, task: Task) -> Result<Gain<Self::Base>> {
        write_part(&self.cusp, |mut cusp| cusp.solve(task))?
    }
}

impl<C> AdaptMid for Edge<C>
where
    C: 'static + AdaptOut + UpdateMut,
{
    fn adapt(&self, deal: &mut dyn Deal) -> Result<()> {
        let (roots, id) = write_part(&self.cusp, |mut cusp| cusp.adapt(deal))??;
        if deal.wrote() {
            for root in &roots {
                root.react(&id)?;
            }
        }
        Ok(())
    }
}

impl<C> Based for Edge<C>
where
    C: 'static + SolveMut + UpdateMut,
    C: AdaptOut + AddRootMut + Debug,
    C::Base: Payload,
{
    type Base = C::Base;
    fn solve(&self, task: Task) -> Result<Gain<Self::Base>> {
        write_part(&self.cusp, |mut cusp| cusp.solve(task))?
    }
    #[cfg(not(feature = "oneThread"))]
    fn backed(&self, back: &Back) -> PloyPointer<Self::Base> {
        Arc::new(RwLock::new(Box::new(Self {
            back: Some(back.clone()),
            cusp: self.cusp.clone(),
        })))
    }
    #[cfg(feature = "oneThread")]
    fn backed(&self, back: &Back) -> PloyPointer<Self::Base> {
        Rc::new(RefCell::new(Box::new(Self {
            back: Some(back.clone()),
            cusp: self.cusp.clone(),
        })))
    }
}

impl<C> BackedMid for Edge<C> {
    fn backed(&self, back: &Back) -> Self {
        Self {
            back: Some(back.clone()),
            cusp: self.cusp.clone(),
        }
    }
}

impl<C, T> WriteBase<T> for Edge<C>
where
    C: WriteBaseOut<T>,
{
    fn write<O, F: FnOnce(&mut T) -> O>(&self, write: F) -> Result<O> {
        let write::Out { roots, id, out } =
            write_part(&self.cusp, |mut cusp| cusp.write_tray_out(write))??;
        for root in &roots {
            root.react(&id)?;
        }
        Ok(out)
    }
}

impl<N> WriteUnit for Edge<N>
where
    N: 'static + WriteUnitOut + UpdateMut,
{
    type Unit = N::Unit;
    fn write<T, F: FnOnce(&mut Pack<Self::Unit>) -> T>(&self, write: F) -> Result<T> {
        let write::Out { roots, id, out } =
            write_part(&self.cusp, |mut cusp| cusp.write_unit_out(write))??;
        for root in &roots {
            root.react(&id)?;
        }
        Ok(out)
    }
}

impl<N> Read for Edge<N>
where
    N: ToItem,
{
    type Item = N::Item;
    fn read<T, F: FnOnce(&Self::Item) -> T>(&self, read: F) -> Result<T> {
        read_part(&self.cusp, |cusp| read(cusp.item()))
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
}

impl<N> React for Edge<N>
where
    N: ReactMut,
{
    fn react(&self, id: &Id) -> react::Result {
        write_part(&self.cusp, |mut cusp| cusp.react(id))?
    }
}
