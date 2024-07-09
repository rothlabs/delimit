use std::{collections::HashMap, hash::Hash};

use crate::*;

pub struct Bare<L> {
    load: L,
}

impl<L> FromItem for Bare<L> {
    type Item = L;
    fn new(load: Self::Item) -> Self {
        Self { load }
    }
}

impl<L> ToLoad for Bare<L>
where
    L: Clone,
{
    type Load = L;
    fn load(&self) -> Self::Load {
        self.load.clone()
    }
}

impl<L> Read for Bare<L> {
    type Unit = L;
    fn read(&self) -> &Self::Unit {
        &self.load
    }
}

impl<L> Write for Bare<L> {
    type Unit = L;
    fn write<F: FnOnce(&mut Self::Unit)>(&mut self, write: F) {
        write(&mut self.load);
    }
}

// impl<L> Clear for Bare<L> {
//     fn clear(&mut self) {}
// }

pub struct Pair<U, L> {
    unit: U,
    load: Option<L>,
}

impl<U, L> FromItem for Pair<U, L> {
    type Item = U;
    fn new(unit: Self::Item) -> Self {
        Self { unit, load: None }
    }
}

impl<U, L> Clear for Pair<U, L> {
    fn clear(&mut self) {
        self.load = None;
    }
}

impl<U, L> WriteWithRoot for Pair<U, L> {
    type Unit = U;
    fn write_with_root<F: FnOnce(&mut Pack<Self::Unit>)>(&mut self, write: F, reactor: &Root) {
        write(&mut Pack {
            unit: &mut self.unit,
            root: reactor,
        });
        self.load = None;
    }
}

impl<U, L> SolveMut for Pair<U, L>
where
    U: Solve<Load = L>,
    L: Clone,
{
    type Load = L;
    fn solve_mut(&mut self) -> Self::Load {
        if let Some(load) = &self.load {
            load.clone()
        } else {
            let load = self.unit.solve();
            self.load = Some(load.clone());
            load
        }
    }
}

#[derive(Default, Clone)]
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

impl<T, L> Clear for Work<T, L> {
    fn clear(&mut self) {
        self.map.clear();
    }
}
