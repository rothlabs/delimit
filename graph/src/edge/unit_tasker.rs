use std::sync::{Arc, RwLock};

use crate::*;

pub struct UnitTasker<U, W> {
    pub root: Option<Reactor>,
    pub stem: Arc<RwLock<node::UnitTasker<U, W>>>,
    pub meta: Meta,
}

impl<U, W> UnitTasker<U, W>
where
    W: Clear,
{
    pub fn writer_with_reactor<F: FnOnce(&mut Pack<U>)>(&self, write: F, reactor: &Reactor) {
        let mut stem = self.stem.write().expect(NO_POISON);
        stem.write_with_reactor(write, reactor);
    }
}

impl<U, W> FromUnit for UnitTasker<U, W>
where
    W: Default,
{
    type Unit = U;
    fn new(unit: Self::Unit) -> Self {
        Self {
            root: None,
            stem: Arc::new(RwLock::new(node::UnitTasker::new(unit))),
            meta: Meta::new(),
        }
    }
}

impl<U, W, T, L> SolveTaskShare<T, L> for UnitTasker<U, W>
where
    U: SolveTask<Task = T, Load = L> + 'static,
    W: Memory<Task = T, Load = L> + 'static,
{
}

impl<U, W> TaskerWithReactor for UnitTasker<U, W>
where
    U: SolveTask<Task = W::Task, Load = W::Load> + 'static,
    W: Memory + 'static,
{
    type Load = W::Load;
    type Task = W::Task;
    fn tasker_with_reactor(
        &self,
        reactor: Reactor,
    ) -> Arc<RwLock<dyn SolveTaskShare<Self::Task, Self::Load>>> {
        Arc::new(RwLock::new(Self {
            root: Some(reactor),
            stem: self.stem.clone(),
            meta: self.meta.clone(),
        }))
    }
}

impl<U, W> ToReactor for UnitTasker<U, W>
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

impl<U, W> WithReactor for UnitTasker<U, W> {
    fn with_reactor(&self, reactor: &Reactor) -> Self {
        Self {
            root: Some(reactor.clone()),
            stem: self.stem.clone(),
            meta: self.meta.clone(),
        }
    }
}

impl<U, W> SolveTask for UnitTasker<U, W>
where
    U: SolveTask<Task = W::Task, Load = W::Load>,
    W: Memory,
{
    type Load = W::Load;
    type Task = W::Task;
    fn solve_task(&self, task: W::Task) -> W::Load {
        let mut stem = self.stem.write().expect(NO_POISON);
        stem.solve_task_mut(task)
    }
}

impl<U, W> Reader for UnitTasker<U, W> {
    type Unit = U;
    fn reader<F: FnOnce(&U)>(&self, read: F) {
        let stem = self.stem.read().expect(NO_POISON);
        read(stem.read());
    }
}

impl<U, W> Writer for UnitTasker<U, W>
where
    W: Clear,
{
    type Unit = U;
    fn writer<F: FnOnce(&mut Self::Unit)>(&self, write: F) {
        let mut stem = self.stem.write().expect(NO_POISON);
        stem.write(write);
    }
}

impl<U, W> AddStem for UnitTasker<U, W> {
    type Unit = U;
    fn add_stem<T, F: FnOnce(&mut U, T)>(&mut self, stem: T, add_stem: F) {
        let mut edge_stem = self.stem.write().expect(NO_POISON);
        edge_stem.add_stem(stem, add_stem);
    }
}

impl<U, W> AddReactor for UnitTasker<U, W> {
    fn add_reactor(&mut self, reactor: Reactor) {
        let mut stem = self.stem.write().expect(NO_POISON);
        stem.add_reactor(reactor);
    }
}

impl<U, W> React for UnitTasker<U, W> {
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
