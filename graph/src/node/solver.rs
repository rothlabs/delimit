use std::{collections::HashMap, hash::Hash};

use crate::New;

use super::{React, Read, Solve, Write};

pub struct Solver<U, T, L> {
    pub unit: U,
    pub work: HashMap<T, L>,
}

impl<U, T, L> New for Solver<U, T, L> {
    type Unit = U;
    fn new(unit: U) -> Self {
        Self {
            unit,
            work: HashMap::new(),
        }
    }
}

impl<U, T, L> Read for Solver<U, T, L> {
    type Unit = U;
    fn read(&self) -> &U {
        &self.unit
    }
}

impl<U, T, L> Write for Solver<U, T, L> {
    type Unit = U;
    fn write<F: FnOnce(&mut U)>(&mut self, write: F) {
        write(&mut self.unit);
    }
}

impl<U, T, L> Solve for Solver<U, T, L>
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

impl<U, T, L> React for Solver<U, T, L>
where
    U: React,
{
    fn react(&mut self) {
        self.unit.react();
    }
}
