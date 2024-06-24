use std::{collections::HashMap, hash::Hash, marker::PhantomData};

use crate::{AddReactor, AddStem, FromUnit, React, Reactor, Solve};

use super::{Read, Write};

pub struct Solver<U, T, L, S> {
    unit: U,
    work: HashMap<T, L>,
    reactors: Vec<Reactor>,
    stem: PhantomData<S>,
}

impl<U, T, L, S> FromUnit for Solver<U, T, L, S> {
    type Unit = U;
    fn new(unit: U) -> Self {
        Self {
            unit,
            work: HashMap::new(),
            reactors: vec![],
            stem: PhantomData {},
        }
    }
}

impl<U, T, L, S> Read for Solver<U, T, L, S> {
    type Unit = U;
    fn read(&self) -> &U {
        &self.unit
    }
}

impl<U, T, L, S> Write for Solver<U, T, L, S> 
where 
    U: React,
{
    type Unit = U;
    fn write<F: FnOnce(&mut U)>(&mut self, write: F) {
        write(&mut self.unit);
        self.react();
    }
}

impl<U, T, L, S> Solve for Solver<U, T, L, S>
where
    U: Solve<Task = T, Load = L>,
    T: Clone + Eq + PartialEq + Hash,
    L: Clone,
{
    type Task = T;
    type Load = L;
    fn solve(&mut self, task: T) -> L {
        if let Some(load) = self.work.get(&task) {
            load.clone()
        } else {
            let load = self.unit.solve(task.clone());
            self.work.insert(task, load.clone());
            load
        }
    }
}

impl<U, T, L, S> React for Solver<U, T, L, S>
where
    U: React,
{
    fn react(&mut self) {
        self.work.clear();
        self.unit.react();
        for reactor in &self.reactors {
            reactor.react();
        }
    }
}

impl<U, T, L, S> AddStem for Solver<U, T, L, S>
where
    U: AddStem<Stem = S> + React,
{
    type Stem = S;
    fn add_stem(&mut self, stem: S) {
        self.unit.add_stem(stem);
        self.react();
    }
}

impl<U, T, L, S> AddReactor for Solver<U, T, L, S> {
    fn add_reactor(&mut self, reactor: &Reactor) {
        self.reactors.push(reactor.clone());
    }
}