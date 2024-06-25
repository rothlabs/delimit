
pub trait FromUnit {
    type Unit;
    fn from_unit(unit: Self::Unit) -> Self;
}

pub trait Work {
    type Task: Clone;
    type Load: Clone;
    fn get(&self, task: &Self::Task) -> Option<Self::Load>;
    fn add(&self, task: Self::Task, load: Self::Load);
}

pub trait AddStem {
    type Stem;
    fn add_stem(&mut self, stem: Self::Stem);
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
