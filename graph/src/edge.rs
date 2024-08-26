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
            back.id.clone()
        } else {
            "".into()
        }
    }
}

// impl<U, N> From<Snap<U>> for Edge<N> 
// where 
//     // U: Default + Solve,
//     Snap<U>: Into<N>,
// {
//     fn from(snap: Snap<U>) -> Self {
//         Self {
//             back: None,
//             #[cfg(not(feature = "oneThread"))]
//             cusp: Arc::new(RwLock::new(snap.into())),
//             #[cfg(feature = "oneThread")]
//             cusp: Rc::new(RefCell::new(snap.into())),
//         }
//     }
// }

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
    N: 'static + Default + MakeInner + DoUpdate,
{
    type Unit = N::Unit;
    #[cfg(not(feature = "oneThread"))]
    fn make<F: FnOnce(&Back) -> Self::Unit>(make: F) -> Self {
        let cusp = N::default();
        let id = cusp.id();
        let cusp = Arc::new(RwLock::new(cusp));
        let update = cusp.clone() as Arc<RwLock<dyn DoUpdate>>;
        let back = Back::new(Arc::downgrade(&update), id);
        write_part(&cusp, |mut cusp| cusp.do_make(make, &back));
        Self { cusp, back: None }
    }
    #[cfg(feature = "oneThread")]
    fn make<F: FnOnce(&Back) -> Self::Unit>(make: F) -> Self {
        let cusp = N::default();
        let id = cusp.id();
        let cusp = Rc::new(RefCell::new(cusp));
        let update = cusp.clone() as Rc<RefCell<dyn DoUpdate>>;
        let back = Back::new(Rc::downgrade(&update), id);
        write_part(&cusp, |mut cusp| cusp.do_make(make, &back));
        Self { cusp, back: None }
    }
}

impl<N> Solve for Edge<N>
where
    N: 'static + DoSolve + DoUpdate,
{
    fn solve(&self, task: Task) -> solve::Result {
        write_part(&self.cusp, |mut cusp| cusp.do_solve(task))
    }
}

impl<N> AdaptInner for Edge<N>
where
    N: 'static + AdaptOut + DoUpdate,
{
    fn adapt(&self, post: Post) -> adapt::Result {
        let write::Out { roots, id, out } = write_part(&self.cusp, |mut cusp| cusp.adapt_out(post));
        for root in &roots {
            root.react(&id)?;
        }
        out
    }
}

impl<U> Engage for Node<U> where U: 'static + Adapt + Solve + Debug + SendSync {}

impl<U> ToPloy for Node<U>
where
    U: 'static + Solve + Adapt + Debug + SendSync,
{
    #[cfg(not(feature = "oneThread"))]
    fn ploy(&self) -> PloyEdge {
        Arc::new(RwLock::new(Box::new(Self {
            back: self.back.clone(),
            cusp: self.cusp.clone(),
        })))
    }
    #[cfg(feature = "oneThread")]
    fn ploy(&self) -> PloyEdge {
        Rc::new(RefCell::new(Box::new(Self {
            back: self.back.clone(),
            cusp: self.cusp.clone(),
        })))
    }
}

impl<U> BackedPloy for Node<U>
where
    U: 'static + Solve + Adapt + Debug + SendSync,
{
    #[cfg(not(feature = "oneThread"))]
    fn backed_ploy(&self, back: &Back) -> PloyEdge {
        Arc::new(RwLock::new(Box::new(Self {
            back: Some(back.clone()),
            cusp: self.cusp.clone(),
        })))
    }
    #[cfg(feature = "oneThread")]
    fn backed_ploy(&self, back: &Back) -> PloyEdge {
        Rc::new(RefCell::new(Box::new(Self {
            back: Some(back.clone()),
            cusp: self.cusp.clone(),
        })))
    }
}

impl<N> ToTray for Edge<N>
where
    N: ToTray,
{
    type Tray = N::Tray;
    fn tray(&self) -> Self::Tray {
        read_part(&self.cusp, |cusp| cusp.tray())
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
    type Item = N::Item;
    fn write<T, F: FnOnce(&mut Self::Item) -> T>(&self, write: F) -> write::Result<T> {
        let write::Out { roots, id, out } =
            write_part(&self.cusp, |mut cusp| cusp.write_tray_out(write));
        for root in &roots {
            root.react(&id)?;
        }
        Ok(out)
    }
}

impl<N> WriteUnit for Edge<N>
where
    N: 'static + WriteUnitOut + DoUpdate,
{
    type Unit = N::Unit;
    fn write<T, F: FnOnce(&mut Pack<Self::Unit>) -> T>(&self, write: F) -> write::Result<T> {
        let write::Out { roots, id, out } =
            write_part(&self.cusp, |mut cusp| cusp.write_unit_out(write));
        for root in &roots {
            root.react(&id)?;
        }
        Ok(out)
    }
}

impl<N> Read for Edge<N>
where
    N: DoRead,
{
    type Item = N::Item;
    fn read<T, F: FnOnce(&Self::Item) -> T>(&self, read: F) -> T {
        read_part(&self.cusp, |cusp| read(cusp.do_read()))
    }
}

impl<N> ReadTray for Edge<N>
where
    N: DoReadTray,
{
    fn read_tray<T, F: FnOnce(tray::ResultRef) -> T>(&self, read: F) -> T {
        read_part(&self.cusp, |cusp| read(cusp.do_read_tray()))
    }
}

impl<N> AddRoot for Edge<N>
where
    N: DoAddRoot,
{
    fn add_root(&self, root: Root) {
        write_part(&self.cusp, |mut cusp| cusp.do_add_root(root));
    }
}

impl<N> Update for Edge<N> where N: SendSync + DoReact {}

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
    N: DoReact,
{
    fn react(&self, id: &Id) -> react::Result {
        write_part(&self.cusp, |mut cusp| cusp.do_react(id))
    }
}

impl AddRoot for Box<dyn Engage> {
    fn add_root(&self, root: Root) {
        self.as_ref().add_root(root)
    }
}

impl Update for Box<dyn Engage> {}

impl ToId for Box<dyn Engage> {
    fn id(&self) -> Id {
        self.as_ref().id()
    }
}

impl Rebut for Box<dyn Engage> {
    fn rebut(&self) -> Ring {
        self.as_ref().rebut()
    }
}

impl React for Box<dyn Engage> {
    fn react(&self, id: &Id) -> react::Result {
        self.as_ref().react(id)
    }
}

impl Solve for Box<dyn Engage> {
    fn solve(&self, task: Task) -> solve::Result {
        self.as_ref().solve(task)
    }
}

impl AdaptInner for Box<dyn Engage> {
    fn adapt(&self, post: Post) -> adapt::Result {
        self.as_ref().adapt(post)
    }
}
