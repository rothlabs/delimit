use crate::*;

pub trait Read {
    type Unit;
    fn read(&self) -> &Self::Unit;
}

impl<T: Unit> Read for T {
    type Unit = T;
    fn read(&self) -> &Self::Unit {
        self
    }
}

pub trait Reader {
    type Unit;
    fn reader<F: FnOnce(&Self::Unit)>(&self, read: F);
}

pub trait CloneUnit {
    type Unit;
    fn unit(&self) -> Self::Unit;
}

pub trait Solve {
    type Task;
    type Load;
    fn solve(&self, task: Self::Task) -> Self::Load;
}
