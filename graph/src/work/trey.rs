// use std::{collections::HashMap, hash::Hash};

// #[derive(Default, Clone)]
// pub struct Trey<U, T, L> {
//     unit: U,
//     map: HashMap<T, L>,
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

// impl<U, T, L> Clear for Trey<U, T, L> {
//     fn clear(&mut self) {
//         self.map.clear();
//     }
// }