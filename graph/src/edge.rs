use super::*;
#[cfg(not(feature = "oneThread"))]
use std::sync::{Arc, RwLock};
#[cfg(feature = "oneThread")]
use std::{cell::RefCell, rc::Rc};

/// Edge to a load.
pub type Leaf = Edge<apex::Leaf>;

/// Edge to a unit that grants a load.
pub type Agent<U> = Edge<apex::Agent<U>>;

/// The forward bridge between nodes.
#[derive(Debug)]
pub struct Edge<N> {
    pub back: Option<Back>,
    #[cfg(not(feature = "oneThread"))]
    pub apex: Arc<RwLock<N>>,
    #[cfg(feature = "oneThread")]
    pub apex: Rc<RefCell<N>>,
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

impl<N> FromItem for Edge<N>
where
    N: FromItem,
{
    type Item = N::Item;
    fn new(unit: Self::Item) -> Self {
        let apex = N::new(unit);
        Self {
            back: None,
            #[cfg(not(feature = "oneThread"))]
            apex: Arc::new(RwLock::new(apex)),
            #[cfg(feature = "oneThread")]
            apex: Rc::new(RefCell::new(apex)),
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
        let apex = N::default();
        let id = apex.id();
        let apex = Arc::new(RwLock::new(apex));
        let update = apex.clone() as Arc<RwLock<dyn DoUpdate>>;
        let back = Back::new(Arc::downgrade(&update), id);
        write_part(&apex, |mut apex| apex.do_make(make, &back));
        Self { apex, back: None }
    }
    #[cfg(feature = "oneThread")]
    fn make<F: FnOnce(&Back) -> Self::Unit>(make: F) -> Self {
        let apex = N::default();
        let id = apex.id();
        let apex = Rc::new(RefCell::new(apex));
        let update = apex.clone() as Rc<RefCell<dyn DoUpdate>>;
        let back = Back::new(Rc::downgrade(&update), id);
        write_part(&apex, |mut apex| apex.do_make(make, &back));
        Self { apex, back: None }
    }
}

impl<N> Solve for Edge<N>
where
    N: 'static + DoSolve + DoUpdate,
{
    fn solve(&self, task: Task) -> solve::Result {
        write_part(&self.apex, |mut apex| apex.do_solve(task))
    }
}

impl<N> AdaptInner for Edge<N>
where
    N: 'static + Adapt + DoUpdate,
{
    fn adapt(&self, post: Post) -> adapt::Result {
        write_part(&self.apex, |mut apex| apex.adapt(post))
    }
}

impl<U> Engage for Agent<U> where U: 'static + Adapt + Solve + Debug + SendSync {}

impl<U> ToPloy for Agent<U>
where
    U: 'static + Solve + Adapt + Debug + SendSync,
{
    #[cfg(not(feature = "oneThread"))]
    fn ploy(&self) -> PloyEdge {
        Arc::new(RwLock::new(Box::new(Self {
            back: self.back.clone(),
            apex: self.apex.clone(),
        })))
    }
    #[cfg(feature = "oneThread")]
    fn ploy(&self) -> PloyEdge {
        Rc::new(RefCell::new(Box::new(Self {
            back: self.back.clone(),
            apex: self.apex.clone(),
        })))
    }
}

impl<U> BackedPloy for Agent<U>
where
    U: 'static + Solve + Adapt + Debug + SendSync,
{
    #[cfg(not(feature = "oneThread"))]
    fn backed_ploy(&self, back: &Back) -> PloyEdge {
        Arc::new(RwLock::new(Box::new(Self {
            back: Some(back.clone()),
            apex: self.apex.clone(),
        })))
    }
    #[cfg(feature = "oneThread")]
    fn backed_ploy(&self, back: &Back) -> PloyEdge {
        Rc::new(RefCell::new(Box::new(Self {
            back: Some(back.clone()),
            apex: self.apex.clone(),
        })))
    }
}

impl<N> ToLoad for Edge<N>
where
    N: ToLoad,
{
    type Load = N::Load;
    fn load(&self) -> Self::Load {
        read_part(&self.apex, |apex| apex.load())
    }
}

impl<N> Backed for Edge<N> {
    fn backed(&self, back: &Back) -> Self {
        Self {
            back: Some(back.clone()),
            apex: self.apex.clone(),
        }
    }
}

impl<N> WriteLoad for Edge<N>
where
    N: WriteLoadOut,
{
    type Item = N::Item;
    fn write<T, F: FnOnce(&mut Self::Item) -> T>(&self, write: F) -> write::Result<T> {
        let write::Out { roots, id, out } =
            write_part(&self.apex, |mut apex| apex.write_load_out(write));
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
            write_part(&self.apex, |mut apex| apex.write_unit_out(write));
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
        read_part(&self.apex, |apex| read(apex.do_read()))
    }
}

impl<N> ReadLoad for Edge<N>
where
    N: DoReadLoad,
{
    fn read_load<T, F: FnOnce(load::ResultRef) -> T>(&self, read: F) -> T {
        read_part(&self.apex, |apex| read(apex.do_read_load()))
    }
}

impl<N> AddRoot for Edge<N>
where
    N: DoAddRoot,
{
    fn add_root(&self, root: Root) {
        write_part(&self.apex, |mut apex| apex.do_add_root(root));
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
        write_part(&self.apex, |mut apex| apex.do_react(id))
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
