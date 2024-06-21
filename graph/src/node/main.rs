use std::{collections::HashMap, hash::Hash, marker::PhantomData};

use serde::Serialize;

use super::{React, Solve, UnitRef};

#[derive(Clone, Serialize)]
pub struct Node<U, T, L, V> {
    pub unit: U,
    pub work: HashMap<T, L>,
    vary: PhantomData<V>,
}

impl<U, T, L, V> Node<U, T, L, V> {
    fn new(unit: U) -> Self {
        Self {
            unit,
            work: HashMap::new(),
            vary: PhantomData {},
        }
    }
}

impl<U, T, L, V> UnitRef for Node<U, T, L, V>
where
    U: Clone,
{
    type Unit = U;
    fn unit(&self) -> &U {
        &self.unit
    }
    fn unit_mut(&mut self) -> &mut U {
        &mut self.unit
    }
}

impl<U, T, L, V> Solve for Node<U, T, L, V>
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
            let load = self.unit.solve(task.clone()); // .expect(LOAD)
            self.work.insert(task, load.clone());
            load
        }
    }
}

impl<U, T, L, V> React for Node<U, T, L, V>
where
    U: React<Vary = V>,
{
    type Vary = V;
    fn react(&mut self, vary: V) {
        self.unit.react(vary);
    }
}
