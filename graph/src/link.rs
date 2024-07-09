use std::sync::{Arc, RwLock};

use crate::*;

pub use leaf::{IntoLeaf, Leaf, ToLeaf};
// pub use solver::Solver;
// pub use tasker::Tasker;
// pub use unit_solver::UnitSolver;
// pub use unit_tasker::UnitTasker;

#[cfg(test)]
mod tests;

mod leaf;
// mod solver;
// mod tasker;
// mod unit_solver;
// mod unit_tasker;

// pub type Solver<L> = Link<Edge<Reactor, dyn SolveShare<L> + 'static>>;
pub type Solver<L> = Link<dyn SolveShare<L>>;

pub type UnitSolver<U, L> = Link<edge::UnitSolver<U, L>>;

pub struct Link<E: ?Sized> {
    meta: Meta,
    edge: Arc<RwLock<E>>,
}

impl<L> Link<dyn SolveShare<L>> {
    pub fn with_reactor(&self, reactor: &Root) -> Self {
        let edge = self.edge.read().expect(NO_POISON);
        Self {
            edge: edge.solver_with_root(reactor.clone()),
            meta: self.meta.clone(),
        }
    }
}

// impl<E> Link<E>
// where
//     E: EventReact + 'static,
// {
//     fn reactor(&self) -> ReactorMut {
//         let edge = self.edge.clone() as Arc<RwLock<dyn EventReact>>;
//         ReactorMut {
//             item: Arc::downgrade(&edge),
//             meta: self.meta.clone(),
//         }
//     }
// }

impl<E> ToLoad for Link<E>
where
    E: ToLoad,
{
    type Load = E::Load;
    fn load(&self) -> Self::Load {
        let edge = self.edge.read().expect(NO_POISON);
        edge.load()
    }
}

impl<E: ?Sized> Clone for Link<E> {
    fn clone(&self) -> Self {
        Self {
            edge: self.edge.clone(),
            meta: self.meta.clone(),
        }
    }
}

impl<E> PartialEq for Link<E> {
    fn eq(&self, other: &Self) -> bool {
        Arc::<RwLock<E>>::ptr_eq(&self.edge, &other.edge) && self.meta == other.meta
    }
}

impl<E> FromItem for Link<E>
where
    E: FromItem,
{
    type Item = E::Item;
    fn new(unit: Self::Item) -> Self {
        Self {
            edge: Arc::new(RwLock::new(E::new(unit))),
            meta: Meta::new(),
        }
    }
}

impl<E> WithRoot for Link<E>
where
    E: WithRoot,
{
    type Root = E::Root;
    fn with_root(&self, root: &Self::Root) -> Self {
        let edge = self.edge.read().expect(NO_POISON);
        Self {
            edge: Arc::new(RwLock::new(edge.with_root(root))),
            meta: self.meta.clone(),
        }
    }
}

impl<E> ToReactor for Link<E>
where
    E: EventReact + 'static,
{
    fn reactor(&self) -> RootEdge {
        let edge = self.edge.clone() as Arc<RwLock<dyn EventReact>>;
        RootEdge {
            item: Arc::downgrade(&edge),
            meta: self.meta.clone(),
        }
    }
}

impl<E: ?Sized> Solve for Link<E>
where
    E: Solve,
{
    type Load = E::Load;
    fn solve(&self) -> Self::Load {
        let edge = self.edge.read().expect(NO_POISON);
        edge.solve()
    }
}

// TODO: use generics on Reader<T> to make multiple implmentations with different bounds
impl<E> Reader for Link<E>
where
    E: Reader + EventReact + AddRoot<Root = RootEdge> + 'static,
{
    type Unit = E::Unit;
    fn reader<F: FnOnce(&Self::Unit)>(&self, read: F) {
        // TODO: first read and check if it is not added as reactor and then write to do so
        let mut edge = self.edge.write().expect(NO_POISON);
        edge.reader(read);
        let reactor = self.reactor();
        edge.add_root(reactor);
    }
}

impl<E> Writer for Link<E>
where
    E: Writer,
{
    type Unit = E::Unit;
    fn writer<F: FnOnce(&mut Self::Unit)>(&self, write: F) {
        let edge = self.edge.read().expect(NO_POISON);
        edge.writer(write);
    }
}

impl<E> WriterWithPack for Link<E>
where
    E: WriterWithPack,
{
    type Unit = E::Unit;
    fn writer<F: FnOnce(&mut Pack<Self::Unit>)>(&self, write: F) {
        let edge = self.edge.read().expect(NO_POISON);
        edge.writer(write);
    }
}

impl<ER, NR, U, L> ToSolver for Link<Edge<ER, Node<NR, Pair<U, L>>>>
where
    ER: 'static,
    NR: 'static,
    U: 'static,
    L: 'static,
    Edge<ER, Node<NR, Pair<U, L>>>: SolveShare<L>,
{
    type Load = L;
    fn solver(&self) -> Solver<L> {
        let edge = self.edge.clone() as Arc<RwLock<dyn SolveShare<L>>>;
        Solver {
            edge,
            meta: self.meta.clone(),
        }
    }
}

// impl<U, L> ToSolver for UnitSolver<U, L>
// where
//     U: Solve<Load = L> + 'static,
//     L: Clone + 'static,
// {
//     type Load = L;
//     fn solver(&self) -> link::Solver<L> {
//         let edge = self.edge.clone() as Arc<RwLock<dyn SolveShare<L>>>;
//         link::Solver {
//             edge,
//             meta: self.meta.clone(),
//         }
//     }
// }

// impl<E> FromLoad for Link<E>
// where
//     E: FromLoad,
// {
//     type Load = E::Load;
//     fn from_load(unit: Self::Load) -> Self {
//         Self {
//             edge: Arc::new(RwLock::new(E::from_load(unit))),
//             meta: Meta::new(),
//         }
//     }
// }

// impl<L> WithReactor for Leaf<L>
// where

// {
//     fn with_reactor(&self, reactor: &Reactor) -> Self {
//         let edge = self.edge.read().expect(NO_POISON);
//         Self {
//             edge: Arc::new(RwLock::new(edge.with_reactor(reactor))),
//             meta: self.meta.clone(),
//         }
//     }
// }

// impl<E> ToReactor for Link<E>
// where
//     E: Clone + React + 'static,
// {
//     fn reactor(&self) -> Reactor {
//         let edge = self.edge.clone() as Arc<RwLock<dyn React>>;
//         Reactor {
//             item: Arc::downgrade(&edge),
//             meta: self.meta.clone(),
//         }
//     }
// }
