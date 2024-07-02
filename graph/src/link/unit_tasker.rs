use std::sync::{Arc, RwLock};

use serde::Serialize;

use crate::*;

pub struct UnitTasker<U, W> {
    edge: Arc<RwLock<edge::UnitTasker<U, W>>>,
    meta: Meta,
}

impl<U, W> Clone for UnitTasker<U, W> {
    fn clone(&self) -> Self {
        Self {
            edge: self.edge.clone(),
            meta: self.meta.clone(),
        }
    }
}

impl<U, W> PartialEq for UnitTasker<U, W> {
    fn eq(&self, other: &Self) -> bool {
        Arc::<RwLock<edge::UnitTasker<U, W>>>::ptr_eq(&self.edge, &other.edge)
            && self.meta == other.meta
    }
}

impl<U, W> FromUnit for UnitTasker<U, W>
where
    W: Default,
{
    type Unit = U;
    fn new(unit: U) -> Self {
        Self {
            edge: Arc::new(RwLock::new(edge::UnitTasker::new(unit))),
            meta: Meta::new(),
        }
    }
}

impl<U, W> ToTasker for UnitTasker<U, W>
where
    U: SolveTask<Task = W::Task, Load = W::Load> + 'static,
    W: Memory + 'static,
{
    type Task = W::Task;
    type Load = W::Load;
    fn to_tasker(&self) -> link::Tasker<W::Task, W::Load> {
        let edge = self.edge.clone() as Arc<RwLock<dyn SolveTaskShare<W::Task, W::Load>>>;
        link::Tasker {
            edge,
            meta: self.meta.clone(),
        }
    }
}

impl<U, W> ToReactor for UnitTasker<U, W>
where
    U: 'static,
    W: 'static,
{
    fn reactor(&self) -> Reactor {
        let edge = self.edge.clone() as Arc<RwLock<dyn React>>;
        Reactor {
            item: Arc::downgrade(&edge),
            meta: self.meta.clone(),
        }
    }
}

impl<U, W> WithReactor for UnitTasker<U, W> {
    fn with_reactor(&self, reactor: &Reactor) -> Self {
        let edge = self.edge.read().expect(NO_POISON);
        Self {
            edge: Arc::new(RwLock::new(edge.with_reactor(reactor))),
            meta: self.meta.clone(),
        }
    }
}

// task solution
impl<U, W> SolveTask for UnitTasker<U, W>
where
    U: SolveTask<Task = W::Task, Load = W::Load>,
    W: Memory,
{
    type Load = U::Load;
    type Task = U::Task;
    fn solve_task(&self, task: U::Task) -> U::Load {
        let edge = self.edge.read().expect(NO_POISON);
        edge.solve_task(task)
    }
}

impl<U, W> Reader for UnitTasker<U, W>
where
    U: React + 'static,
    W: Clear + 'static,
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

impl<U, W> Writer for UnitTasker<U, W>
where
    W: Clear,
{
    type Unit = U;
    fn writer<F: FnOnce(&mut Self::Unit)>(&self, write: F) {
        let edge = self.edge.read().expect(NO_POISON);
        edge.writer(write);
    }
}

impl<U, W> WriterWithReactor for UnitTasker<U, W>
where
    U: React + 'static,
    W: Clear + 'static,
{
    type Unit = U;
    fn writer_with_reactor<F: FnOnce(&mut U, &Reactor)>(&self, write: F) {
        let edge = self.edge.read().expect(NO_POISON);
        edge.writer_with_reactor(write, &edge.reactor());
    }
}

impl<U, W> Stemmer for UnitTasker<U, W>
where
    U: React + 'static,
    W: Clear + 'static,
{
    type Unit = U;
    fn stemmer<T: WithReactor, F: FnOnce(&mut U, T)>(&self, stem: &T, add_stem: F) {
        let mut edge = self.edge.write().expect(NO_POISON);
        let reactor = edge.reactor(); // make a reactor from edge stem
        let stem = stem.with_reactor(&reactor); // make a new link with reactor node
        edge.add_stem(stem, add_stem);
    }
}

impl<U, W> Serialize for UnitTasker<U, W> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.meta.serialize(serializer)
    }
}
