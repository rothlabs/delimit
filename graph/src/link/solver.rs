use std::sync::{Arc, RwLock};

use crate::*;



pub struct Solver<L> {
    pub edge: Arc<RwLock<dyn SolveShare<L>>>,
    pub meta: Meta,
}

impl<L> Clone for Solver<L> {
    fn clone(&self) -> Self {
        Self {
            edge: self.edge.clone(),
            meta: self.meta.clone(),
        }
    }
}

impl<L> Solve for Solver<L> {
    type Load = L;
    fn solve(&self) -> L {
        let edge = self.edge.read().expect(NO_POISON);
        edge.solve()
    }
}

impl<L> WithReactor for Solver<L> {
    fn with_reactor(&self, reactor: &Reactor) -> Self {
        let edge = self.edge.read().expect(NO_POISON);
        Self {
            edge: edge.solver_with_reactor(reactor.clone()),
            meta: self.meta.clone(),
        }
    }
}
