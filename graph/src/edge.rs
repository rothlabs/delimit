use std::hash::Hash;
use std::sync::{Arc, RwLock};

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

/// Edge to a link that grants a link that grants a load.
pub type Pipe<U> = Edge<node::Pipe<U>>;

/// The forward bridge between nodes.
pub struct Edge<N> {
    pub back: Option<Back>,
    pub node: Arc<RwLock<N>>,
    // pub meta: Meta,
}

impl<N> FromItem for Edge<N>
where
    N: FromItem,
{
    type Item = N::Item;
    fn new(unit: Self::Item) -> Self {
        Self {
            back: None,
            node: Arc::new(RwLock::new(N::new(unit))),
            // meta: Meta::new(),
        }
    }
}

impl<N> Make for Edge<N>
where
    N: 'static + Default + DoMake + DoUpdate + Send + Sync,
{
    type Unit = N::Unit;
    fn make<F: FnOnce(&Back) -> Self::Unit>(make: F) -> Self {
        let node = Arc::new(RwLock::new(N::default()));
        let update = node.clone() as Arc<RwLock<dyn DoUpdate + Send + Sync>>;
        let back = Back::new(Arc::downgrade(&update));
        {
            let mut node = node.write().expect(NO_POISON);
            node.do_make(make, &back);
        }
        Self {
            back: None,
            node,
            // meta: Meta::new(),
        }
    }
}

impl<N> Edge<N>
where
    N: 'static + DoUpdate + Send + Sync,
{
    fn node_as_back(&self) -> Back {
        let update = self.node.clone() as Arc<RwLock<dyn DoUpdate + Send + Sync>>;
        Back::new(Arc::downgrade(&update))
    }
}

impl<U, L> Produce<L> for Deuce<U>
where
    U: Grant<Load = L> + 'static + Send + Sync,
    L: Clone + 'static + Send + Sync,
{
}

impl<N> Grant for Edge<N>
where
    N: 'static + DoGrant + DoUpdate + Send + Sync,
{
    type Load = N::Load;
    fn grant(&self) -> Self::Load {
        let mut node = self.node.write().expect(NO_POISON);
        node.do_grant(&self.node_as_back())
    }
}

impl<N> Act for Edge<N>
where
    N: 'static + DoAct + DoUpdate + Send + Sync,
{
    type Load = N::Load;
    fn act(&self) -> Self::Load {
        let mut node = self.node.write().expect(NO_POISON);
        node.do_act(&self.node_as_back())
    }
}

impl<U> ToPloy for Deuce<U>
where
    U: Grant + 'static + Send + Sync,
    U::Load: Clone + 'static + Send + Sync,
{
    type Load = U::Load;
    fn ploy(&self) -> Arc<RwLock<Box<dyn Produce<Self::Load> + Send + Sync>>> {
        Arc::new(RwLock::new(Box::new(Self {
            back: self.back.clone(),
            node: self.node.clone(),
            // meta: self.meta.clone(),
        })))
    }
}

impl<U> BackedPloy for Deuce<U>
where
    U: Grant + 'static + Send + Sync,
    U::Load: Clone + 'static + Send + Sync,
{
    type Load = U::Load;
    fn backed_ploy(&self, back: &Back) -> Arc<RwLock<BoxProduce<U::Load>>> {
        Arc::new(RwLock::new(Box::new(Self {
            back: Some(back.clone()),
            node: self.node.clone(),
            // meta: self.meta.clone(),
        })))
    }
}

impl<U, T, L> Convert<T, L> for Trey<U, T, L>
where
    U: Solve<Task = T, Load = L> + Send + Sync + 'static,
    T: Clone + Eq + PartialEq + Hash + Send + Sync + 'static,
    L: Clone + Send + Sync + 'static,
{
}

impl<N> Solve for Edge<N>
where
    N: DoSolve,
{
    type Task = N::Task;
    type Load = N::Load;
    fn solve(&self, task: Self::Task) -> Self::Load {
        let mut node = self.node.write().expect(NO_POISON);
        node.do_solve(task)
    }
}

impl<U, T, L> ToPlan for Trey<U, T, L>
where
    U: Solve<Task = T, Load = L> + Send + Sync + 'static,
    T: Clone + Eq + PartialEq + Hash + Send + Sync + 'static,
    L: Clone + Send + Sync + 'static,
{
    type Task = T;
    type Load = L;
    fn plan(&self) -> Arc<RwLock<Box<dyn Convert<Self::Task, Self::Load> + Send + Sync>>> {
        Arc::new(RwLock::new(Box::new(Self {
            back: self.back.clone(),
            node: self.node.clone(),
            // meta: self.meta.clone(),
        })))
    }
}

impl<U, T, L> BackedPlan for Trey<U, T, L>
where
    U: Solve<Task = T, Load = L> + Send + Sync + 'static,
    T: Clone + Eq + PartialEq + Hash + Send + Sync + 'static,
    L: Clone + Send + Sync + 'static,
{
    type Task = T;
    type Load = L;
    fn backed_plan(&self, back: &Back) -> Arc<RwLock<BoxConvert<T, L>>> {
        Arc::new(RwLock::new(Box::new(Self {
            back: Some(back.clone()),
            node: self.node.clone(),
            // meta: self.meta.clone(),
        })))
    }
}

impl<N> ToLoad for Edge<N>
where
    N: ToLoad,
{
    type Load = N::Load;
    fn load(&self) -> Self::Load {
        let node = self.node.read().expect(NO_POISON);
        node.load()
    }
}

impl<N> Backed for Edge<N> {
    fn backed(&self, back: &Back) -> Self {
        Self {
            back: Some(back.clone()),
            node: self.node.clone(),
            // meta: self.meta.clone(),
        }
    }
}

impl<N> Write for Edge<N>
where
    N: DoWrite,
{
    type Item = N::Item;
    fn write<F: FnOnce(&mut Self::Item)>(&self, write: F) {
        let mut node = self.node.write().expect(NO_POISON);
        node.do_write(write);
    }
}

impl<N> WriteWithPack for Edge<N>
where
    N: 'static + WriteWithBack + DoUpdate + Send + Sync,
{
    type Unit = N::Unit;
    fn write<F: FnOnce(&mut Pack<Self::Unit>)>(&self, write: F) {
        let mut node = self.node.write().expect(NO_POISON);
        node.write_with_back(write, &self.node_as_back());
    }
}

impl<N> Read for Edge<N>
where
    N: DoRead,
{
    type Item = N::Item;
    fn read<T, F: FnOnce(&Self::Item) -> T>(&self, read: F) -> T {
        let node = self.node.read().expect(NO_POISON);
        read(node.do_read())
    }
}

impl<N> AddRoot for Edge<N>
where
    N: DoAddRoot,
{
    fn add_root(&self, root: Root) {
        let mut node = self.node.write().expect(NO_POISON);
        node.do_add_root(root);
    }
}

impl<N> Update for Edge<N> {}

impl<N> Rebut for Edge<N> {
    fn rebut(&self) -> Ring {
        if let Some(back) = &self.back {
            back.rebut()
        } else {
            Ring::new()
        }
    }
}

impl<N> React for Edge<N> {
    fn react(&self, meta: &Meta) {
        if let Some(back) = &self.back {
            back.react(meta);
        }
    }
}

type BoxProduce<L> = Box<dyn Produce<L> + Send + Sync>;

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
    fn react(&self, meta: &Meta) {
        self.as_ref().react(meta)
    }
}

type BoxConvert<T, L> = Box<dyn Convert<T, L> + Send + Sync>;

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
    fn react(&self, meta: &Meta) {
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
