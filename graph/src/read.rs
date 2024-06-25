pub trait Read {
    type Unit;
    fn read(&self) -> &Self::Unit;
}

pub trait ReadWith {
    type Unit;
    fn read<F: FnOnce(&Self::Unit)>(&self, read: F);
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