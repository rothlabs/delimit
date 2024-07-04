pub trait Read {
    type Unit;
    fn read(&self) -> &Self::Unit;
}

pub trait Reader {
    type Unit;
    fn reader<F: FnOnce(&Self::Unit)>(&self, read: F);
}

pub trait Solve {
    type Load;
    fn solve(&self) -> Self::Load;
}

pub trait SolveTask {
    type Task;
    type Load;
    fn solve_task(&self, task: Self::Task) -> Self::Load;
}


// pub trait CloneUnit {
//     type Unit;
//     fn unit(&self) -> Self::Unit;
// }
