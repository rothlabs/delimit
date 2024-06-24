use std::{collections::HashMap, hash::Hash, marker::PhantomData};

use crate::{base::AddReactor, AddLink, FromUnit, React, Reactor, Solve};

use super::{Read, Write};

pub struct Solver<U, T, L, S> {
    unit: U,
    stem: PhantomData<S>,
    work: HashMap<T, L>,
    reactors: Vec<Reactor>,
}

impl<U, T, L, S> FromUnit for Solver<U, T, L, S> {
    type Unit = U;
    fn new(unit: U) -> Self {
        Self {
            unit,
            stem: PhantomData {},
            work: HashMap::new(),
            reactors: vec![],
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

impl<U, T, L, S> AddLink for Solver<U, T, L, S>
where
    U: AddLink<Link = S> + React,
{
    type Link = S;
    fn add_link(&mut self, link: S) {
        self.unit.add_link(link);
        self.react();
    }
}

impl<U, T, L, S> AddReactor for Solver<U, T, L, S> 
where
    U: React, 
{
    fn add_reactor(&mut self, reactor: &Reactor) {
        self.reactors.push(reactor.clone());
        self.react();
    }
}