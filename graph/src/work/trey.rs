use std::{collections::HashMap, hash::Hash};

use crate::*;

pub struct Trey<U, T, L> {
    unit: U,
    map: HashMap<T, L>,
}

impl<U, T, L> FromItem for Trey<U, T, L> {
    type Item = U;
    fn new(item: Self::Item) -> Self {
        Self {
            unit: item,
            map: HashMap::new(),
        }
    }
}

impl<U, T, L> Solver for Trey<U, T, L>
where
    U: Solve<Task = T, Load = L>,
    T: Clone + Eq + PartialEq + Hash,
    L: Clone,
{
    type Task = T;
    type Load = L;
    fn solver(&mut self, task: T) -> L {
        if let Some(load) = self.map.get(&task) {
            load.clone()
        } else {
            let load = self.unit.solve(task.clone());
            self.map.insert(task, load.clone());
            load
        }
    }
}

impl<U, T, L> Clear for Trey<U, T, L> {
    fn clear(&mut self) {
        self.map.clear();
    }
}

impl<U, T, L> React for Trey<U, T, L> {
    fn react(&mut self, _: &Meta) {}
}

// if let Some(load) = &self.load {
//     load.clone()
// } else {
//     let load = self.unit.grant();
//     self.load = Some(load.clone());
//     load
// }

// impl<U, T, L> Memory for Trey<U, T, L>
// where
//     T: Clone + Eq + PartialEq + Hash,
//     L: Clone,
// {
//     type Load = L;
//     type Task = T;
//     fn add(&mut self, task: Self::Task, load: Self::Load) {
//         self.map.insert(task, load);
//     }
//     fn get(&self, task: &Self::Task) -> Option<&Self::Load> {
//         self.map.get(task)
//     }
// }
