use serde::Serialize;

use crate::*;

#[derive(Clone, Serialize, PartialEq)]
pub struct Solver<U, W>(Link<edge::Solver<U, W>>);

impl<U, W> FromUnit for Solver<U, W>
where
    W: Default,
{
    type Unit = U;
    fn new(unit: U) -> Self {
        Self(Link::new(unit))
    }
}

impl<U, W> WithReactor for Solver<U, W> {
    fn with_reactor(&self, reactor: Reactor) -> Self {
        Self(self.0.with_reactor(reactor))
    }
}

// task solution
impl<U, W> Solve for Solver<U, W>
where
    U: Solve<Task = W::Task, Load = W::Load>,
    W: Memory,
{
    type Load = W::Load;
    type Task = W::Task;
    fn solve(&self, task: W::Task) -> W::Load {
        self.0.solve(task)
    }
}

impl<U, W> Reader for Solver<U, W>
where
    U: Read + React + 'static,
    W: Clear + 'static,
{
    type Unit = U::Unit;
    fn reader<F: FnOnce(&Self::Unit)>(&self, read: F) {
        self.0.reader(read);
    }
}

impl<U, W> Writer for Solver<U, W>
where
    U: Write,
    W: Clear,
{
    type Unit = U::Unit;
    fn writer<F: FnOnce(&mut Self::Unit)>(&self, write: F) {
        self.0.writer(write);
    }
}

impl<U, W> Stemmer for Solver<U, W>
where
    U: React + 'static,
    W: Clear + 'static,
{
    type Unit = U;
    fn stemmer<T: WithReactor, F: FnOnce(&mut Self::Unit, T)>(&self, stem: &T, add_stem: F) {
        self.0.stemmer(stem, add_stem);
    }
}

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
