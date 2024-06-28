use crate::*;

pub trait FromUnit {
    type Unit;
    fn from_unit(unit: Self::Unit) -> Self;
}

pub trait Stemmer {
    type Unit;
    fn stemmer<T: WithReactor, F: FnOnce(&mut Self::Unit, T)>(&self, stem: &T, add_stem: F);
}

pub trait AddStem {
    type Unit;
    fn add_stem<T, F: FnOnce(&mut Self::Unit, T)>(&mut self, stem: T, add_stem: F);
}

pub trait Memory {
    type Load: Clone;
    type Task: Clone;
    fn add(&mut self, task: Self::Task, load: Self::Load);
    fn get(&self, task: &Self::Task) -> Option<&Self::Load>;
}

pub trait Clear {
    fn clear(&mut self);
}

// pub trait Seek {
//     type Root;
//     type Task;
//     fn root(&self) -> &Self::Root;
//     fn task(&self) -> &Self::Task;
// }
