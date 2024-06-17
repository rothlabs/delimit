pub trait Solve<A, G> {
    fn solve(&self, arg: A) -> Option<G>;
}

impl<A, G> Solve<A, G> for String {
    fn solve(&self, task: A) -> Option<G> {
        None
    }
}
