use std::sync::{Arc, RwLock};

use crate::*;

pub struct UnitSolver<U, L> {
    pub root: Option<Reactor>,
    pub stem: Arc<RwLock<node::UnitSolver<U, L>>>,
    pub meta: Meta,
}

impl<U, L> UnitSolver<U, L> {
    pub fn writer_with_reactor<F: FnOnce(&mut WriterPack<U>)>(&self, write: F, reactor: &Reactor) {
        let mut stem = self.stem.write().expect(NO_POISON);
        stem.write_with_reactor(write, reactor);
    }
}

impl<U, L> FromUnit for UnitSolver<U, L> {
    type Unit = U;
    fn new(unit: Self::Unit) -> Self {
        Self {
            root: None,
            stem: Arc::new(RwLock::new(node::UnitSolver::new(unit))),
            meta: Meta::new(),
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
    fn solver_with_reactor(&self, reactor: Reactor) -> Arc<RwLock<dyn SolveShare<Self::Load>>> {
        Arc::new(RwLock::new(Self {
            root: Some(reactor),
            stem: self.stem.clone(),
            meta: self.meta.clone(),
        }))
    }
}

impl<U, L> ToReactor for UnitSolver<U, L>
where
    U: 'static,
    L: 'static,
{
    fn reactor(&self) -> Reactor {
        let stem = self.stem.clone() as Arc<RwLock<dyn React>>;
        Reactor {
            item: Arc::downgrade(&stem),
            meta: self.meta.clone(),
        }
    }
}

impl<U, L> WithReactor for UnitSolver<U, L> {
    fn with_reactor(&self, reactor: &Reactor) -> Self {
        Self {
            root: Some(reactor.clone()),
            stem: self.stem.clone(),
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
        let mut stem = self.stem.write().expect(NO_POISON);
        stem.solve_mut()
    }
}

impl<U, W> Reader for UnitSolver<U, W> {
    type Unit = U;
    fn reader<F: FnOnce(&U)>(&self, read: F) {
        let stem = self.stem.read().expect(NO_POISON);
        read(stem.read());
    }
}

impl<U, L> Writer for UnitSolver<U, L> {
    type Unit = U;
    fn writer<F: FnOnce(&mut Self::Unit)>(&self, write: F) {
        let mut stem = self.stem.write().expect(NO_POISON);
        stem.write(write);
    }
}

impl<U, L> AddStem for UnitSolver<U, L> {
    type Unit = U;
    fn add_stem<T, F: FnOnce(&mut U, T)>(&mut self, stem: T, add_stem: F) {
        let mut edge_stem = self.stem.write().expect(NO_POISON);
        edge_stem.add_stem(stem, add_stem);
    }
}

impl<U, L> AddReactor for UnitSolver<U, L> {
    fn add_reactor(&mut self, reactor: Reactor) {
        let mut stem = self.stem.write().expect(NO_POISON);
        stem.add_reactor(reactor);
    }
}

impl<U, L> React for UnitSolver<U, L> {
    fn clear(&mut self) -> Reactors {
        if let Some(root) = &self.root {
            root.clear()
        } else {
            Reactors::new()
        }
    }
    fn react(&mut self) {
        if let Some(root) = &self.root {
            root.react();
        }
    }
}
