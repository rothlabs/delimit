use std::sync::{Arc, RwLock};

use crate::*;

#[derive(Clone)]
pub struct Solver<T, L>{
    pub edge: Arc<RwLock<dyn Solve<Task = T, Load = L>>>,
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

// impl<T, L> WithReactor for Solver<T, L> {
//     fn with_reactor(&self, reactor: Reactor) -> Self {
//         let edge = self.edge.read().expect(NO_POISON);
//         Self {
//             edge: edge.solver_with_reactor(reactor), //Arc::new(RwLock::new(edge.solver_with_reactor(reactor))),
//             meta: self.meta.clone(),
//         }
//     }
// }

// impl<T, L> SolverWithReactor for UnitSolver<T, L>
// // where
// //     U: Solve<Task = W::Task, Load = W::Load> + 'static,
// //     W: Memory + 'static,
// {
//     type Task = T;
//     type Load = L;
//     fn solver_with_reactor(
//         &self,
//         reactor: Reactor,
//     ) -> Box<dyn SolveReact<T, L>> {
//         Box::new(Self(self.0.with_reactor(reactor)))
//     }
// }



// impl<U, W, T, L> SolveReact<T, L> for Solver<U, W> 
// where
//     U: Solve<Task = T, Load = L> + 'static,
//     W: Memory<Task = T, Load = L> + 'static,
// {}


// impl<U, W> SolverWithReactor for UnitSolver<U, W>
// where
//     U: Solve<Task = W::Task, Load = W::Load> + 'static,
//     W: Memory + 'static,
// {
//     type Load = U::Load;
//     type Task = U::Task;
//     fn solver_with_reactor(
//         &self,
//         reactor: Reactor,
//     ) -> Box<dyn SolveReact<U::Task, U::Load>> {
//         Box::new(Self(self.0.with_reactor(reactor)))
//     }
// }