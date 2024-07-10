use std::sync::{Arc, RwLock};

use crate::*;

pub struct Tasker<T, L> {
    pub edge: Arc<RwLock<dyn SolveTaskShare<T, L>>>,
    pub meta: Meta,
}

impl<T, L> SolveTask for Tasker<T, L> {
    type Task = T;
    type Load = L;
    fn solve_task(&self, task: T) -> L {
        let edge = self.edge.read().expect(NO_POISON);
        edge.solve_task(task)
    }
}

impl<T, L> WithReactor for Tasker<T, L> {
    fn with_reactor(&self, reactor: &Reactor) -> Self {
        let edge = self.edge.read().expect(NO_POISON);
        Self {
            edge: edge.tasker_with_reactor(reactor.clone()),
            meta: self.meta.clone(),
        }
    }
}
