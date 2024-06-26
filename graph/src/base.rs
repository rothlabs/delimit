use crate::link::Leaf;


pub trait FromUnit {
    type Unit;
    fn from_unit(unit: Self::Unit) -> Self;
}

pub trait FromUnit2 {
    fn from_unit2<T>(unit: T) -> Self;
}

pub trait AddStem {
    type Stem;
    fn add_stem(&mut self, stem: Self::Stem);
}

pub trait Work {
    type Task: Clone;
    type Load: Clone;
    fn get(&self, task: &Self::Task) -> Option<Self::Load>;
    fn add(&self, task: Self::Task, load: Self::Load);
}

pub trait Clear {
    fn clear(&self);
}

// pub trait Seek {
//     type Root;
//     type Task;
//     fn root(&self) -> &Self::Root;
//     fn task(&self) -> &Self::Task;
// }
