use std::{collections::HashMap, hash::Hash, marker::PhantomData};

use crate::FromUnit;

use super::{AddLink, React, Read, Solve, Write};

pub struct Solver<U, T, L, S> {
    pub unit: U,
    pub work: HashMap<T, L>,
    stem: PhantomData<S>,
}

impl<U, T, L, S> FromUnit for Solver<U, T, L, S> {
    type Unit = U;
    fn new(unit: U) -> Self {
        Self {
            unit,
            work: HashMap::new(),
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

impl<U, T, L, S> Write for Solver<U, T, L, S> {
    type Unit = U;
    fn write<F: FnOnce(&mut U)>(&mut self, write: F) {
        write(&mut self.unit);
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
        self.unit.react();
    }
}

impl<U, T, L, S> AddLink for Solver<U, T, L, S> {
    type Unit = U;
    type Link = S;
    fn add_link<F: FnOnce(&mut U, S)>(&mut self, stem: S, add: F) {
        add(&mut self.unit, stem);
    }
}
