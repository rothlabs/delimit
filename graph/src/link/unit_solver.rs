use std::sync::{Arc, RwLock};

use serde::Serialize;

use crate::*;

pub struct UnitSolver<U, L> {
    edge: Arc<RwLock<edge::UnitSolver<U, L>>>,
    meta: Meta,
}

impl<U, W> Clone for UnitSolver<U, W> {
    fn clone(&self) -> Self {
        Self {
            edge: self.edge.clone(),
            meta: self.meta.clone(),
        }
    }
}

impl<U, W> PartialEq for UnitSolver<U, W> {
    fn eq(&self, other: &Self) -> bool {
        Arc::<RwLock<edge::UnitSolver<U, W>>>::ptr_eq(&self.edge, &other.edge)
            && self.meta == other.meta
    }
}

impl<U, L> FromUnit for UnitSolver<U, L> {
    type Unit = U;
    fn new(unit: U) -> Self {
        Self {
            edge: Arc::new(RwLock::new(edge::UnitSolver::new(unit))),
            meta: Meta::new(),
        }
    }
}

impl<U, L> ToSolver for UnitSolver<U, L>
where
    U: Solve<Load = L> + 'static,
    L: Clone + 'static,
{
    type Load = L;
    fn solver(&self) -> link::Solver<L> {
        let edge = self.edge.clone() as Arc<RwLock<dyn SolveShare<L>>>;
        link::Solver {
            edge,
            meta: self.meta.clone(),
        }
    }
}

impl<U, L> ToReactor for UnitSolver<U, L>
where
    U: 'static,
    L: 'static,
{
    fn reactor(&self) -> Reactor {
        let edge = self.edge.clone() as Arc<RwLock<dyn React>>;
        Reactor {
            item: Arc::downgrade(&edge),
            meta: self.meta.clone(),
        }
    }
}

impl<U, W> WithReactor for UnitSolver<U, W> {
    fn with_reactor(&self, reactor: &Reactor) -> Self {
        let edge = self.edge.read().expect(NO_POISON);
        Self {
            edge: Arc::new(RwLock::new(edge.with_reactor(reactor))),
            meta: self.meta.clone(),
        }
    }
}

impl<U, L> Solve for UnitSolver<U, L>
where
    U: Solve<Load = L>,
    L: Clone,
{
    type Load = L;
    fn solve(&self) -> L {
        let edge = self.edge.read().expect(NO_POISON);
        edge.solve()
    }
}

impl<U, L> Reader for UnitSolver<U, L>
where
    U: React + 'static,
    L: 'static,
{
    type Unit = U;
    fn reader<F: FnOnce(&U)>(&self, read: F) {
        // TODO: first read and check if it is not added as reactor and then write to do so
        let mut edge = self.edge.write().expect(NO_POISON);
        edge.reader(read);
        let reactor = self.reactor();
        edge.add_reactor(reactor);
    }
}

impl<U, L> WriterWithPack for UnitSolver<U, L>
where
    U: 'static,
    L: 'static,
{
    type Unit = U;
    fn writer<F: FnOnce(&mut Pack<U>)>(&self, write: F) {
        let edge = self.edge.read().expect(NO_POISON);
        edge.writer_with_reactor(write, &edge.reactor());
    }
}

impl<U, L> Stemmer for UnitSolver<U, L>
where
    U: 'static,
    L: 'static,
{
    type Unit = U;
    fn stemmer<T: WithReactor, F: FnOnce(&mut U, T)>(&self, stem: &T, add_stem: F) {
        let mut edge = self.edge.write().expect(NO_POISON);
        let reactor = edge.reactor(); // make a reactor from edge stem
        let stem = stem.with_reactor(&reactor); // make a new link with reactor node
        edge.add_stem(stem, add_stem);
    }
}

impl<U, W> Serialize for UnitSolver<U, W> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.meta.serialize(serializer)
    }
}

// impl<U, L> Writer for UnitSolver<U, L> {
//     type Unit = U;
//     fn writer<F: FnOnce(&mut Self::Unit)>(&self, write: F) {
//         let edge = self.edge.read().expect(NO_POISON);
//         edge.writer(write);
//     }
// }
