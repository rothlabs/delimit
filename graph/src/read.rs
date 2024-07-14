pub trait Read {
    type Item;
    fn read(&self) -> &Self::Item;
}

pub trait Reader {
    type Item;
    fn reader<F: FnOnce(&Self::Item)>(&self, read: F);
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