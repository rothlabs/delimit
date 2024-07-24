use std::hash::Hash;
#[cfg(not(feature = "oneThread"))]
use std::sync::{Arc, RwLock};
#[cfg(feature = "oneThread")]
use std::{cell::RefCell, rc::Rc};

use crate::*;

/// Edge to a load.
pub type Ace<L> = Edge<node::Ace<L>>;

/// Edge to a unit that grants a load.
pub type Deuce<U> = Edge<node::Deuce<U>>;

/// Edge to a unit that solves a task with resulting load.
pub type Trey<U, T, L> = Edge<node::Trey<U, T, L>>;

/// Edge to a unit that grants a load.
/// Unlike Deuce, the agent will act upon some external system.
pub type Agent<U> = Edge<node::Agent<U>>;

/// Edge to a unit that solves a task and could act upon externals.
pub type Envoy<U> = Edge<node::Envoy<U>>;

/// Edge to a link that grants a link that grants a load.
pub type Pipe<U> = Edge<node::Pipe<U>>;

/// The forward bridge between nodes.
pub struct Edge<N> {
    pub meta: Meta,
    pub back: Option<Back>,
    #[cfg(not(feature = "oneThread"))]
    pub node: Arc<RwLock<N>>,
    #[cfg(feature = "oneThread")]
    pub node: Rc<RefCell<N>>,
}

impl<N> ToMeta for Edge<N> {
    fn meta(&self) -> Meta {
        self.meta.clone()
    }
}

impl<N> FromItem for Edge<N>
where
    N: FromItem + ToMeta,
{
    type Item = N::Item;
    fn new(unit: Self::Item) -> Self {
        let node = N::new(unit);
        Self {
            meta: node.meta(),
            back: None,
            #[cfg(not(feature = "oneThread"))]
            node: Arc::new(RwLock::new(node)),
            #[cfg(feature = "oneThread")]
            node: Rc::new(RefCell::new(node)),
        }
    }
}

impl<N> Make for Edge<N>
where
    N: 'static + Default + DoMake + DoUpdate + ToMeta,
{
    type Unit = N::Unit;
    #[cfg(not(feature = "oneThread"))]
    fn make<F: FnOnce(&Back) -> Self::Unit>(make: F) -> Self {
        let node = N::default();
        let meta = node.meta();
        let node = Arc::new(RwLock::new(node));
        let update = node.clone() as Arc<RwLock<dyn DoUpdate>>;
        let back = Back::new(Arc::downgrade(&update));
        write_part(&node, |mut node| node.do_make(make, &back));
        Self {
            meta,
            back: None,
            node,
        }
    }
    #[cfg(feature = "oneThread")]
    fn make<F: FnOnce(&Back) -> Self::Unit>(make: F) -> Self {
        let node = N::default();
        let meta = node.meta();
        let node = Rc::new(RefCell::new(node));
        let update = node.clone() as Rc<RefCell<dyn DoUpdate>>;
        let back = Back::new(Rc::downgrade(&update));
        write_part(&node, |mut node| node.do_make(make, &back));
        Self {
            meta,
            node,
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
        let update = self.node.clone() as Arc<RwLock<dyn DoUpdate>>;
        Back::new(Arc::downgrade(&update))
    }
    #[cfg(feature = "oneThread")]
    fn node_as_back(&self) -> Back {
        let update = self.node.clone() as Rc<RefCell<dyn DoUpdate>>;
        Back::new(Rc::downgrade(&update))
    }
}

impl<U, L> Produce<L> for Deuce<U>
where
    U: 'static + Grant<Load = L> + SendSync,
    L: 'static + Clone + SendSync,
{
}

impl<N> Grant for Edge<N>
where
    N: 'static + DoGrant + DoUpdate,
{
    type Load = N::Load;
    fn grant(&self) -> Self::Load {
        write_part(&self.node, |mut node| node.do_grant(&self.node_as_back()))
    }
}

impl<N> Act for Edge<N>
where
    N: 'static + DoAct + DoUpdate,
{
    type Load = N::Load;
    fn act(&self) -> Self::Load {
        write_part(&self.node, |mut node| node.do_act(&self.node_as_back()))
    }
}

impl<U> ToPloy for Deuce<U>
where
    U: 'static + Grant + SendSync,
    U::Load: 'static + Clone + SendSync,
{
    type Load = U::Load;
    #[cfg(not(feature = "oneThread"))]
    fn ploy(&self) -> Arc<RwLock<Box<dyn Produce<Self::Load>>>> {
        Arc::new(RwLock::new(Box::new(Self {
            meta: self.meta(),
            back: self.back.clone(),
            node: self.node.clone(),
        })))
    }
    #[cfg(feature = "oneThread")]
    fn ploy(&self) -> Rc<RefCell<Box<dyn Produce<Self::Load>>>> {
        //  + Send + Sync
        Rc::new(RefCell::new(Box::new(Self {
            meta: self.meta(),
            back: self.back.clone(),
            node: self.node.clone(),
        })))
    }
}

impl<U> BackedPloy for Deuce<U>
where
    U: 'static + Grant + SendSync,
    U::Load: 'static + Clone + SendSync,
{
    type Load = U::Load;
    #[cfg(not(feature = "oneThread"))]
    fn backed_ploy(&self, back: &Back) -> Arc<RwLock<BoxProduce<U::Load>>> {
        Arc::new(RwLock::new(Box::new(Self {
            meta: self.meta(),
            back: Some(back.clone()),
            node: self.node.clone(),
        })))
    }
    #[cfg(feature = "oneThread")]
    fn backed_ploy(&self, back: &Back) -> Rc<RefCell<BoxProduce<U::Load>>> {
        Rc::new(RefCell::new(Box::new(Self {
            meta: self.meta(),
            back: Some(back.clone()),
            node: self.node.clone(),
        })))
    }
}

impl<U, T, L> Convert<T, L> for Trey<U, T, L>
where
    U: 'static + Solve<Task = T, Load = L> + SendSync,
    T: 'static + Clone + Eq + PartialEq + Hash + SendSync,
    L: 'static + Clone + SendSync,
{
}

impl<N> Solve for Edge<N>
where
    N: DoSolve,
{
    type Task = N::Task;
    type Load = N::Load;
    fn solve(&self, task: Self::Task) -> Self::Load {
        write_part(&self.node, |mut node| node.do_solve(task))
    }
}

impl<N> Serve for Edge<N>
where
    N: DoServe,
{
    type Task = N::Task;
    type Load = N::Load;
    fn serve(&self, task: Self::Task) -> Self::Load {
        write_part(&self.node, |mut node| node.do_serve(task))
    }
}

impl<U, T, L> ToPlan for Trey<U, T, L>
where
    U: 'static + Solve<Task = T, Load = L> + SendSync,
    T: 'static + Clone + Eq + PartialEq + Hash + SendSync,
    L: 'static + Clone + SendSync,
{
    type Task = T;
    type Load = L;
    #[cfg(not(feature = "oneThread"))]
    fn plan(&self) -> Arc<RwLock<Box<dyn Convert<Self::Task, Self::Load>>>> {
        Arc::new(RwLock::new(Box::new(Self {
            meta: self.meta(),
            back: self.back.clone(),
            node: self.node.clone(),
        })))
    }
    #[cfg(feature = "oneThread")]
    fn plan(&self) -> Rc<RefCell<Box<dyn Convert<Self::Task, Self::Load>>>> {
        Rc::new(RefCell::new(Box::new(Self {
            meta: self.meta(),
            back: self.back.clone(),
            node: self.node.clone(),
        })))
    }
}

impl<U, T, L> BackedPlan for Trey<U, T, L>
where
    U: 'static + Solve<Task = T, Load = L> + SendSync,
    T: 'static + Clone + Eq + PartialEq + Hash + SendSync,
    L: 'static + Clone + SendSync,
{
    type Task = T;
    type Load = L;
    #[cfg(not(feature = "oneThread"))]
    fn backed_plan(&self, back: &Back) -> Arc<RwLock<BoxConvert<T, L>>> {
        Arc::new(RwLock::new(Box::new(Self {
            meta: self.meta(),
            back: Some(back.clone()),
            node: self.node.clone(),
        })))
    }
    #[cfg(feature = "oneThread")]
    fn backed_plan(&self, back: &Back) -> Rc<RefCell<BoxConvert<T, L>>> {
        Rc::new(RefCell::new(Box::new(Self {
            meta: self.meta(),
            back: Some(back.clone()),
            node: self.node.clone(),
        })))
    }
}

impl<N> ToLoad for Edge<N>
where
    N: ToLoad,
{
    type Load = N::Load;
    fn load(&self) -> Self::Load {
        read_part(&self.node, |node| node.load())
    }
}

impl<N> Backed for Edge<N> {
    fn backed(&self, back: &Back) -> Self {
        Self {
            meta: self.meta(),
            back: Some(back.clone()),
            node: self.node.clone(),
        }
    }
}

impl<N> Write for Edge<N>
where
    N: WriteWithRoots,
{
    type Item = N::Item;
    fn write<T, F: FnOnce(&mut Self::Item) -> T>(&self, write: F) -> T {
        let (roots, meta, out) = write_part(&self.node, |mut node| node.write_with_roots(write));
        for root in &roots {
            root.react(&meta);
        }
        out
    }
}

impl<N> WriteWithPack for Edge<N>
where
    N: 'static + WriteWithBackRoots + DoUpdate,
{
    type Unit = N::Unit;
    fn write<T, F: FnOnce(&mut Pack<Self::Unit>) -> T>(&self, write: F) -> T {
        let (roots, meta, out) = write_part(&self.node, |mut node| {
            node.write_with_back_roots(write, &self.node_as_back())
        });
        for root in &roots {
            root.react(&meta);
        }
        out
    }
}

impl<N> Read for Edge<N>
where
    N: DoRead,
{
    type Item = N::Item;
    fn read<T, F: FnOnce(&Self::Item) -> T>(&self, read: F) -> T {
        read_part(&self.node, |node| read(node.do_read()))
    }
}

impl<N> AddRoot for Edge<N>
where
    N: DoAddRoot,
{
    fn add_root(&self, root: Root) {
        write_part(&self.node, |mut node| node.do_add_root(root));
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
    fn react(&self, meta: &Meta) -> ReactResult {
        write_part(&self.node, |mut node| node.do_react(meta))
    }
}

type BoxProduce<L> = Box<dyn Produce<L>>;

impl<L> Grant for BoxProduce<L> {
    type Load = L;
    fn grant(&self) -> Self::Load {
        self.as_ref().grant()
    }
}

impl<L> AddRoot for BoxProduce<L> {
    fn add_root(&self, root: Root) {
        self.as_ref().add_root(root)
    }
}

impl<L> Update for BoxProduce<L> {}

impl<L> Rebut for BoxProduce<L> {
    fn rebut(&self) -> Ring {
        self.as_ref().rebut()
    }
}

impl<L> React for BoxProduce<L> {
    fn react(&self, meta: &Meta) -> ReactResult {
        self.as_ref().react(meta)
    }
}

type BoxConvert<T, L> = Box<dyn Convert<T, L>>;

impl<T, L> Solve for BoxConvert<T, L> {
    type Task = T;
    type Load = L;
    fn solve(&self, task: Self::Task) -> Self::Load {
        self.as_ref().solve(task)
    }
}

impl<T, L> AddRoot for BoxConvert<T, L> {
    fn add_root(&self, root: Root) {
        self.as_ref().add_root(root)
    }
}

impl<T, L> Update for BoxConvert<T, L> {}

impl<T, L> Rebut for BoxConvert<T, L> {
    fn rebut(&self) -> Ring {
        self.as_ref().rebut()
    }
}

impl<T, L> React for BoxConvert<T, L> {
    fn react(&self, meta: &Meta) -> ReactResult {
        self.as_ref().react(meta)
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
//         write_part(&self.node, |mut node| node.do_write(write))
//     }
// }
