#[cfg(not(feature = "oneThread"))]
use std::sync::{Arc, RwLock};
#[cfg(feature = "oneThread")]
use std::{cell::RefCell, rc::Rc};

use serde::Serialize;

use crate::*;

/// Edge to a load.
pub type Leaf = Edge<apex::Leaf>;

/// Edge to a unit that grants a load.
pub type Agent<U> = Edge<apex::Agent<U>>;

/// The forward bridge between nodes.
pub struct Edge<N> {
    pub meta: Meta,
    pub back: Option<Back>,
    #[cfg(not(feature = "oneThread"))]
    pub apex: Arc<RwLock<N>>,
    #[cfg(feature = "oneThread")]
    pub apex: Rc<RefCell<N>>,
}

impl<N> ToMeta for Edge<N> {
    fn meta(&self) -> Meta {
        self.meta.clone()
    }
}

impl<N> SerializeGraph for Edge<N> 
where 
    N: SerializeGraph
{
    fn serial(&self, serial: &mut Serial) -> serial::Result {
        read_part(&self.apex, |apex| apex.serial(serial))
    }
}

impl<N> FromItem for Edge<N>
where
    N: FromItem + ToMeta,
{
    type Item = N::Item;
    fn new(unit: Self::Item) -> Self {
        let apex = N::new(unit);
        Self {
            meta: apex.meta(),
            back: None,
            #[cfg(not(feature = "oneThread"))]
            apex: Arc::new(RwLock::new(apex)),
            #[cfg(feature = "oneThread")]
            apex: Rc::new(RefCell::new(apex)),
        }
    }
}

impl<N> Maker for Edge<N>
where
    N: 'static + Default + DoMake + DoUpdate + ToMeta,
{
    type Unit = N::Unit;
    #[cfg(not(feature = "oneThread"))]
    fn maker<F: FnOnce(&Back) -> Self::Unit>(make: F) -> Self {
        let apex = N::default();
        let meta = apex.meta();
        let apex = Arc::new(RwLock::new(apex));
        let update = apex.clone() as Arc<RwLock<dyn DoUpdate>>;
        let back = Back::new(Arc::downgrade(&update));
        write_part(&apex, |mut apex| apex.do_make(make, &back));
        Self {
            meta,
            back: None,
            apex,
        }
    }
    #[cfg(feature = "oneThread")]
    fn maker<F: FnOnce(&Back) -> Self::Unit>(make: F) -> Self {
        let apex = N::default();
        let meta = apex.meta();
        let apex = Rc::new(RefCell::new(apex));
        let update = apex.clone() as Rc<RefCell<dyn DoUpdate>>;
        let back = Back::new(Rc::downgrade(&update));
        write_part(&apex, |mut apex| apex.do_make(make, &back));
        Self {
            meta,
            apex,
            back: None,
        }
    }
}

impl<N> Edge<N>
where
    N: 'static + DoUpdate,
{
    #[cfg(not(feature = "oneThread"))]
    fn node_as_back(&self) -> Back {
        let update = self.apex.clone() as Arc<RwLock<dyn DoUpdate>>;
        Back::new(Arc::downgrade(&update))
    }
    #[cfg(feature = "oneThread")]
    fn node_as_back(&self) -> Back {
        let update = self.apex.clone() as Rc<RefCell<dyn DoUpdate>>;
        Back::new(Rc::downgrade(&update))
    }
}

impl<N> Solve for Edge<N>
where
    N: 'static + DoSolve + DoUpdate,
{
    fn solve(&self, task: Task) -> solve::Result {
        write_part(&self.apex, |mut apex| {
            apex.do_solve(task, &self.node_as_back())
        })
    }
}

impl<N> DoAlter for Edge<N>
where
    N: 'static + Alter + DoUpdate,
{
    fn alter(&self, post: Post) -> alter::Result {
        let back = &self.node_as_back();
        write_part(&self.apex, |mut apex| apex.alter(post.backed(back)))
    }
}

impl<U> Engage for Agent<U> where U: 'static + Solve + Alter + Serialize + SendSync {}

impl<U> ToPloy for Agent<U>
where
    U: 'static + Solve + Alter + Serialize + SendSync,
{
    #[cfg(not(feature = "oneThread"))]
    fn ploy(&self) -> PloyEdge {
        Arc::new(RwLock::new(Box::new(Self {
            meta: self.meta(),
            back: self.back.clone(),
            apex: self.apex.clone(),
        })))
    }
    #[cfg(feature = "oneThread")]
    fn ploy(&self) -> PloyEdge {
        //  + Send + Sync
        Rc::new(RefCell::new(Box::new(Self {
            meta: self.meta(),
            back: self.back.clone(),
            apex: self.apex.clone(),
        })))
    }
}

impl<U> BackedPloy for Agent<U>
where
    U: 'static + Solve + Alter + Serialize + SendSync,
{
    #[cfg(not(feature = "oneThread"))]
    fn backed_ploy(&self, back: &Back) -> PloyEdge {
        Arc::new(RwLock::new(Box::new(Self {
            meta: self.meta(),
            back: Some(back.clone()),
            apex: self.apex.clone(),
        })))
    }
    #[cfg(feature = "oneThread")]
    fn backed_ploy(&self, back: &Back) -> PloyEdge {
        Rc::new(RefCell::new(Box::new(Self {
            meta: self.meta(),
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
            meta: self.meta(),
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
        let write::Out { roots, meta, out } =
            write_part(&self.apex, |mut apex| apex.write_load_out(write));
        for root in &roots {
            root.react(&meta)?;
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
        let write::Out { roots, meta, out } = write_part(&self.apex, |mut apex| {
            apex.write_unit_out(write, &self.node_as_back())
        });
        for root in &roots {
            root.react(&meta)?;
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
    fn react(&self, meta: &Meta) -> react::Result {
        write_part(&self.apex, |mut apex| apex.do_react(meta))
    }
}

impl SerializeGraph for Box<dyn Engage> {
    fn serial(&self, serial: &mut Serial) -> serial::Result {
        self.as_ref().serial(serial)
    }
}

impl AddRoot for Box<dyn Engage> {
    fn add_root(&self, root: Root) {
        self.as_ref().add_root(root)
    }
}

impl Update for Box<dyn Engage> {}

impl Rebut for Box<dyn Engage> {
    fn rebut(&self) -> Ring {
        self.as_ref().rebut()
    }
}

impl React for Box<dyn Engage> {
    fn react(&self, meta: &Meta) -> react::Result {
        self.as_ref().react(meta)
    }
}

impl Solve for Box<dyn Engage> {
    fn solve(&self, task: Task) -> solve::Result {
        self.as_ref().solve(task)
    }
}

impl DoAlter for Box<dyn Engage> {
    fn alter(&self, post: Post) -> alter::Result {
        self.as_ref().alter(post)
    }
}

// impl<R, St> Serialize for Edge<R, St> {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         self.meta.serialize(serializer)
//     }
// }

// impl<N> Write for Edge<N>
// where
//     N: DoWrite,
// {
//     type Item = N::Item;
//     fn write<T, F: FnOnce(&mut Self::Item) -> T>(&self, write: F) -> T {
//         write_part(&self.apex, |mut apex| apex.do_write(write))
//     }
// }
