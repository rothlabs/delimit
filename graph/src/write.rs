use crate::*;

pub trait Write {
    type Unit;
    fn write<F: FnOnce(&mut Self::Unit)>(&mut self, write: F);
}

pub trait Writer {
    type Unit;
    fn writer<F: FnOnce(&mut Self::Unit)>(&self, write: F);
}

pub trait WriteWithReactor {
    type Unit;
    fn write_with_reactor<F: FnOnce(&mut Self::Unit, &Reactor)>(
        &mut self,
        write: F,
        reactor: &Reactor,
    );
}

pub trait WriterWithReactor {
    type Unit;
    fn writer_with_reactor<F: FnOnce(&mut Self::Unit, &Reactor)>(&self, write: F);
}

pub trait SolveMut {
    type Load;
    fn solve_mut(&mut self) -> Self::Load;
}

pub trait SolveTaskMut {
    type Task;
    type Load;
    fn solve_task_mut(&mut self, task: Self::Task) -> Self::Load;
}


