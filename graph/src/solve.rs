use crate::Stem;

pub trait Solve<T, G> {
    fn solve(&self, task: T) -> Option<G>;
    fn stems(&self) -> Vec<Box<dyn Stem>>;
}

impl<T, G> Solve<T, G> for String {
    fn solve(&self, _: T) -> Option<G> {
        None
    }
    fn stems(&self) -> Vec<Box<dyn Stem>> {
        vec![]
    }
}
