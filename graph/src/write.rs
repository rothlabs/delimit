use crate::*;

pub trait Write {
    type Unit;
    fn write<F: FnOnce(&mut Self::Unit)>(&mut self, write: F);
}

pub trait Writer {
    type Unit;
    fn write<F: FnOnce(&mut Self::Unit)>(&self, write: F);
}

impl<T: Unit> Write for T {
    type Unit = T;
    fn write<F: FnOnce(&mut T)>(&mut self, write: F) {
        write(self)
    }
}

pub trait SolveMut {
    type Task;
    type Load;
    fn solve_mut(&mut self, task: Self::Task) -> Self::Load;
}
