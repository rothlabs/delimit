use std::{collections::HashMap, hash::Hash, marker::PhantomData};

use serde::Serialize;

use crate::SolveReact;

const LOAD: &str = "there should be a load";

#[derive(Clone, Serialize)]
pub struct Node<U, T, L, V> {
    pub unit: U,
    pub work: HashMap<T, L>,
    _vary: PhantomData<V>,
}

impl<U, T, L, V> Node<U, T, L, V> 
where
    U: Clone + SolveReact<T, L, V>,
    T: Clone + Eq + PartialEq + Hash,
    L: Clone,
{
    pub fn new(unit: U) -> Self {
        Self {
            unit,
            work: HashMap::new(),
            _vary: PhantomData{},
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
    pub fn react(&mut self, vary: V) {
        self.unit.react(vary);
    }
}

// pub fn flatten(&self, flat: &mut Flat) { // , state: &mut Hasher
//     let unit = serde_json::to_string(&self.read()).unwrap();
// }
// TODO: pub fn duplicate(&self) -> Node<U>  // clone and set new ID
