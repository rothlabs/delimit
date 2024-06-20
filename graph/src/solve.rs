pub trait SolveReact<T, L, V> {
    fn solve(&self, task: T) -> Option<L>;
    fn react(&self, vary: V);
}

impl<T, L, V> SolveReact<T, L, V> for String {
    fn solve(&self, _: T) -> Option<L> {
        None
    }
    fn react(&self, _: V) {}
}

// fn stems(&self) -> Vec<Box<dyn Stem>>;
// fn stems(&self) -> Vec<Box<dyn Stem>> {
//     vec![]
// }
