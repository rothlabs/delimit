use std::sync::{Arc, RwLock};

use serde::Serialize;

use crate::*;

#[derive(Clone)]
pub struct UnitSolver<U, W>{
    edge: Arc<RwLock<edge::Solver<U, W>>>,
    meta: Meta,
}

impl<U, W> PartialEq for UnitSolver<U, W> {
    fn eq(&self, other: &Self) -> bool {
        Arc::<RwLock<edge::Solver<U, W>>>::ptr_eq(&self.edge, &other.edge) && self.meta == other.meta
    }
}

impl<U, W> FromUnit for UnitSolver<U, W>
where
    W: Default,
{
    type Unit = U;
    fn new(unit: U) -> Self {
        Self {
            edge: Arc::new(RwLock::new(edge::Solver::new(unit))),
            meta: Meta::new(),
        }
    }
}

impl<U, W> WithReactor for UnitSolver<U, W> {
    fn with_reactor(&self, reactor: Reactor) -> Self {
        let edge = self.edge.read().expect(NO_POISON);
        Self {
            edge: Arc::new(RwLock::new(edge.with_reactor(reactor))),
            meta: self.meta.clone(),
        }
    }
}

impl<U, W> ToReactor for UnitSolver<U, W>
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

// task solution
impl<U, W> Solve for UnitSolver<U, W>
where
    U: Solve<Task = W::Task, Load = W::Load>,
    W: Memory,
{
    type Load = U::Load;
    type Task = U::Task;
    fn solve(&self, task: U::Task) -> U::Load {
        let edge = self.edge.read().expect(NO_POISON);
        edge.solve(task)
    }
}

impl<U, W> Reader for UnitSolver<U, W>
where
    U: Read + React + 'static,
    W: Clear + 'static,
{
    type Unit = U::Unit;
    fn reader<F: FnOnce(&Self::Unit)>(&self, read: F) {
        // TODO: first read and check if it is not added as reactor and then write to do so
        let mut edge = self.edge.write().expect(NO_POISON);
        edge.reader(read);
        let reactor = self.reactor();
        edge.add_reactor(reactor);
    }
}

impl<U, W> Writer for UnitSolver<U, W>
where
    U: Write,
    W: Clear,
{
    type Unit = U::Unit;
    fn writer<F: FnOnce(&mut Self::Unit)>(&self, write: F) {
        let edge = self.edge.read().expect(NO_POISON);
        edge.writer(write);
    }
}

impl<U, W> Stemmer for UnitSolver<U, W>
where
    U: React + 'static,
    W: Clear + 'static,
{
    type Unit = U;
    fn stemmer<T: WithReactor, F: FnOnce(&mut Self::Unit, T)>(&self, stem: &T, add_stem: F) {
        let mut edge = self.edge.write().expect(NO_POISON);
        let reactor_node = edge.reactor(); // make a reactor from edge stem
        let stem = stem.with_reactor(reactor_node); // make a new link with reactor node
        edge.add_stem(stem, add_stem);
    }
}

impl<U, W> Serialize for  UnitSolver<U, W>  {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.meta.serialize(serializer)
    }
}

// impl<U, W> StemSolver for UnitSolver<U, W> 
// where 
//     U: Solve<Task = W::Task, Load = W::Load>,
//     U: React + 'static,
//     W: Memory + Clear + 'static,
// {
//     type Unit = U;
//     type Load = U::Load;
//     type Task = U::Task;
//     fn stem_solver<T: SolveReact<Self::Task, Self::Load>, F: FnOnce(&mut Self::Unit, Box<dyn SolveReact<Self::Task, Self::Load>>)>(&self, stem: &T, add_stem: F) {
//         self.0.stem_solver(stem, add_stem);
//     }
// }



// impl<U, W> Serialize for Solver<U, W> {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         self.0.serialize(serializer)
//     }
// }

// impl<U, W> AddStem for Solver<U, W>
// where
//     edge::Solver<U, W>: AddStem,
// {
//     type Stem = <edge::Solver<U, W> as AddStem>::Stem;
//     fn add_stem(&mut self, stem: Self::Stem) {
//         self.0.add_stem(stem);
//     }
// }

// impl<U, T, L, S> Clone for Solver<U, T, L, S> {
//     fn clone(&self) -> Self {
//         Self(self.0.clone())
//     }
// }

// impl<U, T, L, S> Clone for Solver<U, T, L, S> {
//     fn clone(&self) -> Self {
//         Self {
//             edge: self.edge.clone(),
//             meta: self.meta.clone(),
//         }
//     }
// }

// impl<U, T, L, S> Serialize for Solver<U, T, L, S> {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         self.meta.serialize(serializer)
//     }
// }

// let edge = self.edge.read().expect(NO_POISON);
// edge.solve(task)
