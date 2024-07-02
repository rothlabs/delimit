use std::sync::{Arc, RwLock};

use crate::*;

#[derive(Clone)]
pub struct Solver<U, W>{
    pub root: Option<Reactor>,
    pub stem: Arc<RwLock<node::UnitSolver<U, W>>>,
    pub meta: Meta,
}

impl<U, W> FromUnit for Solver<U, W>
where
    W: Default,
{
    type Unit = U;
    fn new(unit: Self::Unit) -> Self {
        Self {
            root: None,
            stem: Arc::new(RwLock::new(node::UnitSolver::new(unit))),
            meta: Meta::new(),
        }
    }
}

impl<U, W, T, L> SolveReact<T, L> for Solver<U, W> 
where
    U: Solve<Task = T, Load = L> + 'static,
    W: Memory<Task = T, Load = L> + 'static,
{}


impl<U, W> SolverWithReactor for Solver<U, W>
where
    U: Solve<Task = W::Task, Load = W::Load> + 'static,
    W: Memory + 'static,
{
    type Load = W::Load;
    type Task = W::Task;
    fn solver_with_reactor(
            &self,
            reactor: Reactor,
        ) -> Arc<RwLock<dyn SolveReact<Self::Task, Self::Load>>> {
        let wow = Self {
            root: Some(reactor),
            stem: self.stem.clone(),
            meta: self.meta.clone(),
        };
        Arc::new(RwLock::new(wow))
    }
}

impl<U, W> ToReactor for Solver<U, W>
where
    U: React + 'static,
    W: Clear + 'static,
{
    fn reactor(&self) -> Reactor {
        let stem = self.stem.clone() as Arc<RwLock<dyn React>>;
        Reactor {
            item: Arc::downgrade(&stem),
            meta: self.meta.clone(),
        }
    }
}

impl<U, W> WithReactor for Solver<U, W> {
    fn with_reactor(&self, reactor: Reactor) -> Self {
        Self {
            root: Some(reactor),
            stem: self.stem.clone(),
            meta: self.meta.clone(),
        }
    }
}

impl<U, W> Solve for Solver<U, W>
where
    U: Solve<Task = W::Task, Load = W::Load>,
    W: Memory,
{
    type Load = W::Load;
    type Task = W::Task;
    fn solve(&self, task: W::Task) -> W::Load {
        let mut stem = self.stem.write().expect(NO_POISON);
        stem.solve_mut(task)
    }
}

impl<U, W> Reader for Solver<U, W>
where
    U: Read,
{
    type Unit = U::Unit;
    fn reader<F: FnOnce(&Self::Unit)>(&self, read: F) {
        let stem = self.stem.read().expect(NO_POISON);
        read(stem.read());
    }
}

impl<U, W> Writer for Solver<U, W>
where
    U: Write,
    W: Clear,
{
    type Unit = U::Unit;
    fn writer<F: FnOnce(&mut Self::Unit)>(&self, write: F) {
        let mut stem = self.stem.write().expect(NO_POISON);
        stem.write(write);
    }
}

impl<U, W> AddStem for Solver<U, W> {
    type Unit = U;
    fn add_stem<T, F: FnOnce(&mut U, T)>(&mut self, stem: T, add_stem: F) {
        let mut edge_stem = self.stem.write().expect(NO_POISON);
        edge_stem.add_stem(stem, add_stem);
    }
}

impl<U, W> AddReactor for Solver<U, W> {
    fn add_reactor(&mut self, reactor: Reactor) {
        let mut stem = self.stem.write().expect(NO_POISON);
        stem.add_reactor(reactor);
    }
}

impl<U, W> React for Solver<U, W> {
    fn clear(&mut self) -> Reactors {
        if let Some(root) = &self.root {
            root.clear()
        } else {
            Reactors::default()
        }
    }
    fn react(&mut self) {
        if let Some(root) = &self.root {
            root.react();
        }
    }
}
