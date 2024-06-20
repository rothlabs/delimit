pub trait Solve<T, L> {
    fn solve(&self, task: T) -> Option<L>;
}

impl<T, L> Solve<T, L> for String {
    fn solve(&self, _: T) -> Option<L> {
        None
    }
}

// fn stems(&self) -> Vec<Box<dyn Stem>>;
// fn stems(&self) -> Vec<Box<dyn Stem>> {
//     vec![]
// }
