use crate::*;
use std::{collections::HashMap, hash::Hash};

/// For task-resolution units that may act upon externals
pub struct Envoy<U>
where
    U: Serve,
{
    unit: U,
    map: HashMap<U::Task, U::Load>,
}

impl<U> FromItem for Envoy<U>
where
    U: Serve,
{
    type Item = U;
    fn new(item: Self::Item) -> Self {
        Self {
            unit: item,
            map: HashMap::new(),
        }
    }
}

impl<U> DoServe for Envoy<U>
where
    U: Serve,
    U::Task: Clone + Eq + PartialEq + Hash,
    U::Load: Clone,
{
    type Task = U::Task;
    type Load = U::Load;
    fn do_serve(&mut self, task: Self::Task) -> Self::Load {
        if let Some(load) = self.map.get(&task) {
            load.clone()
        } else {
            let load = self.unit.serve(task.clone());
            self.map.insert(task, load.clone());
            load
        }
    }
}

impl<U> Clear for Envoy<U>
where
    U: Serve,
{
    fn clear(&mut self) {
        self.map.clear();
    }
}

impl<U> DoReact for Envoy<U>
where
    U: Serve,
{
    fn do_react(&mut self, _: &Meta) {}
}

// if let Some(load) = &self.load {
//     load.clone()
// } else {
//     let load = self.unit.grant();
//     self.load = Some(load.clone());
//     load
// }

// impl<U, T, L> Memory for Envoy<U, T, L>
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
