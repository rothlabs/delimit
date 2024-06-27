pub trait FromUnit {
    type Unit;
    fn from_unit(unit: Self::Unit) -> Self;
}

pub trait AddStem {
    type Stem;
    fn add_stem(&mut self, stem: Self::Stem);
}

pub trait Memory {
    type Load: Clone;
    type Task: Clone;
    fn add(&mut self, task: Self::Task, load: Self::Load);
    fn get(&self, task: &Self::Task) -> Option<&Self::Load>;
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
