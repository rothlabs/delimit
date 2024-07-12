use std::{collections::HashMap, hash::Hash};

use crate::*;

pub struct Trey<U, T, L> {
    unit: U,
    map: HashMap<T, L>,
}

impl<U, T, L> SolveTaskMut for Trey<U, T, L>
where
    U: SolveTask<Task = T, Load = L>,
    T: Clone + Eq + PartialEq + Hash,
    L: Clone,
{
    type Task = T;
    type Load = L;
    fn solve_task_mut(&mut self, task: T) -> L {
        if let Some(load) = self.map.get(&task) {
            load.clone()
        } else {
            let load = self.unit.solve_task(task.clone());
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

// if let Some(load) = &self.load {
//     load.clone()
// } else {
//     let load = self.unit.solve();
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