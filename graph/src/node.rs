use std::{collections::HashMap, hash::Hash};

use serde::Serialize;

use crate::Solve;

const LOAD: &str = "there should be a load";

#[derive(Clone, Serialize)]
pub struct Node<U, T, L> {
    pub unit: U,
    pub work: HashMap<T, L>,
}

impl<U: Solve<T, L>, T: Clone + Eq + PartialEq + Hash, L: Clone> Node<U, T, L> {
    pub fn new(unit: U) -> Self {
        Self {
            unit,
            work: HashMap::new(),
        }
    }
    pub fn solve(&mut self, task: T) -> L {
        if let Some(load) = self.work.get(&task) {
            load.clone()
        } else {
            let load = self.unit.solve(task.clone()).expect(LOAD);
            self.work.insert(task, load.clone());
            load
        }
    }
}

// pub fn flatten(&self, flat: &mut Flat) { // , state: &mut Hasher
//     let unit = serde_json::to_string(&self.read()).unwrap();
// }
// TODO: pub fn duplicate(&self) -> Node<U>  // clone and set new ID
