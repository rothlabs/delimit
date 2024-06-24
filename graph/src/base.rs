use std::sync::{Arc, RwLock};

pub trait FromUnit {
    type Unit;
    fn new(unit: Self::Unit) -> Self;
}

pub trait FromRoot {
    type Root;
    fn from_root(&self, root: &Arc<RwLock<Self::Root>>) -> Self;
}

pub trait AddLink { 
    type Link;
    fn add_link(&mut self, link: Self::Link);
}


pub trait Solve {
    type Task;
    type Load;
    fn solve(&mut self, task: Self::Task) -> Self::Load;
}

pub trait React {
    fn react(&mut self); 
}
