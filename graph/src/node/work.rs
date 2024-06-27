use std::{collections::HashMap, hash::Hash};

use crate::*;

#[derive(Default)]
pub struct Work<T, L> {
    map: HashMap<T, L>,
}

impl<T, L> Memory for Work<T, L>
where
    T: Clone + Eq + PartialEq + Hash,
    L: Clone,
{
    type Load = L;
    type Task = T;
    fn add(&mut self, task: Self::Task, load: Self::Load) {
        self.map.insert(task, load);
    }
    fn get(&self, task: &Self::Task) -> Option<&Self::Load> {
        self.map.get(task)
    }
}
