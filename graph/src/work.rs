use std::{collections::HashMap, hash::Hash};

use crate::*;

pub struct UnitLoad<U, L> {
    unit: U,
    load: Option<L>,
}

impl<U, L> FromUnit for UnitLoad<U, L> 
{
    type Unit = U;
    fn from_unit(unit: Self::Unit) -> Self {
        Self {
            unit,
            load: None,
        }
    }
}

impl<U, L> Write for UnitLoad<U, L> {
    type Unit = U;
    fn write<F: FnOnce(&mut Self::Unit)>(&mut self, write: F) {
        write(&mut self.unit);
    }
}

impl<U, L> WriteWithReactor for UnitLoad<U, L> {
    type Unit = U;
    fn write_with_reactor<F: FnOnce(&mut Pack<Self::Unit>)>(
            &mut self,
            write: F,
            reactor: &Reactor,
        ) {
        write(&mut Pack {
            unit: &mut self.unit,
            reactor,
        });
        self.load = None;
    }
}

impl<U, L> SolveMut for UnitLoad<U, L> 
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
