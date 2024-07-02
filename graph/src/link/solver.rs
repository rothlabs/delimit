use std::sync::{Arc, RwLock};

use crate::*;

#[derive(Clone)]
pub struct Solver<T, L>{
    pub edge: Arc<RwLock<dyn SolveReact<T, L>>>,
    pub meta: Meta,
}

impl<T, L> Solve for Solver<T, L> {
    type Task = T;
    type Load = L;
    fn solve(&self, task: T) -> L {
        let edge = self.edge.read().expect(NO_POISON);
        edge.solve(task)
    }
}

impl<T, L> WithReactor for Solver<T, L> {
    fn with_reactor(&self, reactor: Reactor) -> Self {
        let edge = self.edge.read().expect(NO_POISON);
        Self {
            edge: edge.solver_with_reactor(reactor), 
            meta: self.meta.clone(),
        }
    }
}
