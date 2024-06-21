use std::{collections::HashMap, hash::Hash, marker::PhantomData};

use serde::Serialize;

use crate::SolveReact;

use super::{Node, LOAD};

#[derive(Clone, Serialize)]
pub struct Main<U, T, L, V> {
    pub unit: U,
    pub work: HashMap<T, L>,
    vary: PhantomData<V>,
}

impl<U, T, L, V> Node for Main<U, T, L, V> 
where
    U: Clone + SolveReact<T, L, V>,
    T: Clone + Eq + PartialEq + Hash,
    L: Clone,
{
    type U = U;
    type T = T;
    type L = L;
    type V = V;
    fn new(unit: U) -> Self {
        Self {
            unit,
            work: HashMap::new(),
            vary: PhantomData{},
        }
    }
    fn unit(&self) -> &Self::U {
        &self.unit
    }
    fn unit_mut(&mut self) -> &mut Self::U {
        &mut self.unit
    }
    fn solve(&mut self, task: T) -> L {
        if let Some(load) = self.work.get(&task) {
            load.clone()
        } else {
            let load = self.unit.solve(task.clone()).expect(LOAD);
            self.work.insert(task, load.clone());
            load
        }
    }
    fn react(&mut self, vary: V) {
        self.unit.react(vary);
    }
}