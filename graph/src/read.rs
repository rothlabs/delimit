pub trait Read {
    type Unit;
    fn read(&self) -> &Self::Unit;
}

pub trait Reader {
    type Unit; // TODO: rename to item because it can refer to load or unit?
    fn reader<F: FnOnce(&Self::Unit)>(&self, read: F);
}

pub trait Grant {
    type Load;
    fn grant(&self) -> Self::Load;
}

pub trait Solve {
    type Task;
    type Load;
    fn solve(&self, task: Self::Task) -> Self::Load;
}

// pub trait CloneUnit {
//     type Unit;
//     fn unit(&self) -> Self::Unit;
// }
