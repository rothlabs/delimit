use std::sync::{Arc, RwLock};

use react::{Event, EventMut, EventReactMut};

use crate::*;

// pub use leaf::Leaf;
// pub use unit_solver::UnitSolver; 
// pub use unit_tasker::UnitTasker;

// mod leaf;
// mod unit_solver;
// mod unit_tasker;

pub type Leaf<U> = Edge<RootNode, node::Leaf<U>>;

pub type UnitSolver<U, L> = Edge<RootNode, node::UnitSolver<U, L>>;

pub struct Edge<R, S> {
    pub root: Option<R>,
    pub stem: Arc<RwLock<S>>,
    pub meta: Meta,
}

impl<R, S> Edge<R, S>
where
    S: EventReactMut + 'static,
{
    fn reactor(&self) -> RootNode {
        let stem = self.stem.clone() as Arc<RwLock<dyn EventReactMut>>;
        RootNode {
            item: Arc::downgrade(&stem),
            meta: self.meta.clone(),
        }
    }
}

impl<U, L> SolveShare<L> for UnitSolver<U, L>
where
    U: Solve<Load = L> + 'static,
    L: Clone + 'static,
{
}

impl<U, L> SolverWithReactor for UnitSolver<U, L>
where
    U: Solve<Load = L> + 'static,
    L: Clone + 'static,
{
    type Load = L;
    fn solver_with_reactor(&self, reactor: RootNode) -> Arc<RwLock<dyn SolveShare<Self::Load>>> {
        Arc::new(RwLock::new(Self {
            root: Some(reactor),
            stem: self.stem.clone(),
            meta: self.meta.clone(),
        }))
    }
}

impl<R, S> ToLoad for Edge<R, S> 
where 
    S: ToLoad,
{
    type Load = S::Load;
    fn load(&self) -> Self::Load {
        let stem = self.stem.read().expect(NO_POISON);
        stem.load()
    }
}

impl<R, S> FromItem for Edge<R, S> 
where 
    S: FromItem,
{
    type Item = S::Item;
    fn new(unit: Self::Item) -> Self {
        Self {
            root: None,
            stem: Arc::new(RwLock::new(S::new(unit))),
            meta: Meta::new(),
        }
    }
}

impl<R, S> WithRoot for Edge<R, S> 
where 
    R: Clone,
{ 
    type Root = R;
    fn with_root(&self, root: &R) -> Self {
        Self {
            root: Some(root.clone()),
            stem: self.stem.clone(),
            meta: self.meta.clone(),
        }
    }
}

// impl<R, S> ToReactor for Edge<R, S>
// where
//     S: EventReactMut + 'static,
// {
//     fn reactor(&self) -> Reactor {
//         let stem = self.stem.clone() as Arc<RwLock<dyn EventReactMut>>;
//         Reactor {
//             item: Arc::downgrade(&stem),
//             meta: self.meta.clone(),
//         }
//     }
// }

impl<R, S> Writer for Edge<R, S> 
where 
    S: Write,
{
    type Unit = S::Unit;
    fn writer<F: FnOnce(&mut Self::Unit)>(&self, write: F) {
        let mut stem = self.stem.write().expect(NO_POISON);
        stem.write(write);
    }
}

impl<R, S> WriterWithPack for Edge<R, S> 
where 
    S: WriteWithReactor + EventReactMut + 'static,
{
    type Unit = S::Unit;
    fn writer<F: FnOnce(&mut Pack<Self::Unit>)>(&self, write: F) {
        let mut stem = self.stem.write().expect(NO_POISON);
        stem.write_with_reactor(write, &self.reactor());
    }
}

// impl<R, S> WriterWithReactor for Edge<R, S> 
// where 
//     S: WriteWithReactor
// {
//     type Unit = S::Unit;
//     fn writer_with_reactor<F: FnOnce(&mut Pack<Self::Unit>)>(&self, write: F, reactor: &Reactor) {
//         let mut stem = self.stem.write().expect(NO_POISON);
//         stem.write_with_reactor(write, reactor);
//     }
// }

impl<R, S> Reader for Edge<R, S> 
where
    S: Read,
{
    type Unit = S::Unit;
    fn reader<F: FnOnce(&Self::Unit)>(&self, read: F) {
        let stem = self.stem.read().expect(NO_POISON);
        read(stem.read());
    }
}

impl<R, S> Solve for Edge<R, S>
where
    S: SolveMut 
{
    type Load = S::Load;
    fn solve(&self) -> Self::Load {
        let mut stem = self.stem.write().expect(NO_POISON);
        stem.solve_mut()
    }
}

impl<R, S> EventReact for Edge<R, S> 
where 
    R: Event<Roots = Reactors> + React,
{}

impl<R, S> Event for Edge<R, S> 
where 
    R: Event<Roots = Reactors>,
{
    type Roots = R::Roots;
    fn event(&self) -> Self::Roots {
        if let Some(root) = &self.root {
            root.event()
        } else {
            Reactors::new()
        }
    }    
}

impl<R, S> React for Edge<R, S> 
where 
    R: React,
{
    fn react(&self) {
        if let Some(root) = &self.root {
            root.react();
        }
    }
}

impl<R, S> AddRoot for Edge<R, S>  
where 
    S: AddRoot,
{
    type Root = S::Root;
    fn add_root(&mut self, root: Self::Root) {
        let mut stem = self.stem.write().expect(NO_POISON);
        stem.add_root(root);
    }
}



// impl<R, S> FromLoad for Edge<R, S> 
// where 
//     S: FromLoad,
// {
//     type Load = S::Load;
//     fn from_load(unit: Self::Load) -> Self {
//         Self {
//             root: None,
//             stem: Arc::new(RwLock::new(S::from_load(unit))),
//             meta: Meta::new(),
//         }
//     }
// }


// impl<R, S> Clone for Edge<R, S> 
// where 
//     R: Clone,
// {
//     fn clone(&self) -> Self {
//         Self {
//             root: self.root.clone(),
//             stem: self.stem.clone(),
//             meta: self.meta.clone(),
//         }
//     }
// }