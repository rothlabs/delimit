use super::*;
#[cfg(not(feature = "oneThread"))]
use std::sync::{Arc, RwLock};
#[cfg(feature = "oneThread")]
use std::{cell::RefCell, rc::Rc};

/// Edge to a tray.
pub type Leaf = Edge<cusp::Leaf>;

/// Edge to a unit that grants a tray.
pub type Node<U> = Edge<cusp::Node<U>>;

/// The forward bridge between apexes.
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

impl<N> Make for Edge<N>
where
    N: 'static + Default + MakeMid + UpdateMid,
{
    type Unit = N::Unit;
    #[cfg(not(feature = "oneThread"))]
    fn make<F: FnOnce(&Back) -> Self::Unit>(make: F) -> Self {
        let cusp = N::default();
        let id = cusp.id();
        let cusp = Arc::new(RwLock::new(cusp));
        let update = cusp.clone() as Arc<RwLock<dyn UpdateMid>>;
        let back = Back::new(Arc::downgrade(&update), id);
        write_part(&cusp, |mut cusp| cusp.make(make, &back)).expect(IMMEDIATE_ACCESS);
        Self { cusp, back: None }
    }
    #[cfg(feature = "oneThread")]
    fn make<F: FnOnce(&Back) -> Self::Unit>(make: F) -> Self {
        let cusp = N::default();
        let id = cusp.id();
        let cusp = Rc::new(RefCell::new(cusp));
        let update = cusp.clone() as Rc<RefCell<dyn UpdateMid>>;
        let back = Back::new(Rc::downgrade(&update), id);
        write_part(&cusp, |mut cusp| cusp.make(make, &back)).expect(IMMEDIATE_ACCESS);
        Self { cusp, back: None }
    }
}

// impl<N> MakePloy for Edge<N>
// where
//     N: 'static + Default + MakeMid + UpdateMid,
// {
//     type Unit = N::Unit;
//     #[cfg(not(feature = "oneThread"))]
//     fn make_ploy<F: FnOnce(&Back) -> Self::Unit>(make: F) -> PloyPointer {
//         let cusp = N::default();
//         let id = cusp.id();
//         let cusp = Arc::new(RwLock::new(cusp));
//         let update = cusp.clone() as Arc<RwLock<dyn UpdateMid>>;
//         let back = Back::new(Arc::downgrade(&update), id);
//         write_part(&cusp, |mut cusp| cusp.make(make, &back)).expect(IMMEDIATE_ACCESS);
//         PloyPointer { cusp, back: None }
//     }
//     #[cfg(feature = "oneThread")]
//     fn make<F: FnOnce(&Back) -> Self::Unit>(make: F) -> Self {
//         let cusp = N::default();
//         let id = cusp.id();
//         let cusp = Rc::new(RefCell::new(cusp));
//         let update = cusp.clone() as Rc<RefCell<dyn UpdateMid>>;
//         let back = Back::new(Rc::downgrade(&update), id);
//         write_part(&cusp, |mut cusp| cusp.make(make, &back)).expect(IMMEDIATE_ACCESS);
//         Self { cusp, back: None }
//     }
// }

impl<U> ToPloy for Node<U>
where
    U: 'static + Solve + Adapt + Debug + SendSync,
{
    #[cfg(not(feature = "oneThread"))]
    fn ploy(&self) -> PloyPointer {
        Arc::new(RwLock::new(Box::new(Self {
            back: self.back.clone(),
            cusp: self.cusp.clone(),
        })))
    }
    #[cfg(feature = "oneThread")]
    fn ploy(&self) -> PloyPointer {
        Rc::new(RefCell::new(Box::new(Self {
            back: self.back.clone(),
            cusp: self.cusp.clone(),
        })))
    }
}

impl<N> FromSnap for Edge<N>
where
    N: 'static + Default + WithSnap + UpdateMid,
{
    type Unit = N::Unit;
    #[cfg(not(feature = "oneThread"))]
    fn from_snap(snap: Snap<Self::Unit>) -> Self {
        let cusp = N::default();
        let id = cusp.id();
        let cusp = Arc::new(RwLock::new(cusp));
        let update = cusp.clone() as Arc<RwLock<dyn UpdateMid>>;
        let back = Back::new(Arc::downgrade(&update), id);
        write_part(&cusp, |mut cusp| cusp.with_snap(snap, &back)).expect(IMMEDIATE_ACCESS);
        Self { cusp, back: None }
    }
    #[cfg(feature = "oneThread")]
    fn from_snap(snap: Snap<Self::Unit>) -> Self {
        let cusp = N::default();
        let id = cusp.id();
        let cusp = Rc::new(RefCell::new(cusp));
        let update = cusp.clone() as Rc<RefCell<dyn UpdateMid>>;
        let back = Back::new(Rc::downgrade(&update), id);
        write_part(&cusp, |mut cusp| cusp.with_snap(snap, &back)).expect(IMMEDIATE_ACCESS);
        Self { cusp, back: None }
    }
}

impl<N> Solve for Edge<N>
where
    N: 'static + DoSolve + UpdateMid,
{
    fn solve(&self, task: Task) -> solve::Result {
        write_part(&self.cusp, |mut cusp| cusp.do_solve(task))?
    }
}

impl<N> AdaptMid for Edge<N>
where
    N: 'static + AdaptOut + UpdateMid,
{
    fn adapt(&self, post: Post) -> adapt::Result {
        let write::Out { roots, id, out } = write_part(&self.cusp, |mut cusp| cusp.adapt(post))??;
        for root in &roots {
            root.react(&id)?;
        }
        Ok(out)
    }
}

impl<U> BackedPloy for Node<U>
where
    U: 'static + Solve + Adapt + Debug + SendSync,
{
    #[cfg(not(feature = "oneThread"))]
    fn backed_ploy(&self, back: &Back) -> PloyPointer {
        Arc::new(RwLock::new(Box::new(Self {
            back: Some(back.clone()),
            cusp: self.cusp.clone(),
        })))
    }
    #[cfg(feature = "oneThread")]
    fn backed_ploy(&self, back: &Back) -> PloyPointer {
        Rc::new(RefCell::new(Box::new(Self {
            back: Some(back.clone()),
            cusp: self.cusp.clone(),
        })))
    }
}

impl<N> Backed for Edge<N> {
    fn backed(&self, back: &Back) -> Self {
        Self {
            back: Some(back.clone()),
            cusp: self.cusp.clone(),
        }
    }
}

impl<N> WriteTray for Edge<N>
where
    N: WriteTrayOut,
{
    fn write<T, F: FnOnce(&mut Tray) -> T>(&self, write: F) -> Result<T> {
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
    N: 'static + WriteUnitOut + UpdateMid,
{
    type Unit = N::Unit;
    fn write<T, F: FnOnce(&mut Pack<Self::Unit>) -> T>(
        &self,
        write: F,
    ) -> Result<T> {
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
    type Payload = N::Item;
    fn read<T, F: FnOnce(&Self::Payload) -> T>(&self, read: F) -> Result<T> {
        read_part(&self.cusp, |cusp| read(cusp.item()))
    }
}

impl<N> AddRoot for Edge<N>
where
    N: AddRootMut,
{
    fn add_root(&self, root: Root) {
        // TODO: propagate error up
        write_part(&self.cusp, |mut cusp| cusp.add_root(root)).ok();
    }
}

impl<N> Rebut for Edge<N> {
    fn rebut(&self) -> Ring {
        if let Some(back) = &self.back {
            back.rebut()
        } else {
            Ring::new()
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
