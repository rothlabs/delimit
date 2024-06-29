use crate::*;

#[derive(Clone)]
pub struct Solver<U, W>(Edge<node::Solver<U, W>>);

impl<U, W> FromUnit for Solver<U, W>
where
    W: Default,
{
    type Unit = U;
    fn new(unit: Self::Unit) -> Self {
        Self(edge::Edge::new(unit))
    }
}

impl<U, W> ToReactor for Solver<U, W>
where
    U: React + 'static,
    W: Clear + 'static,
{
    fn reactor(&self) -> Reactor {
        self.0.reactor()
    }
}

impl<U, W> WithReactor for Solver<U, W> {
    fn with_reactor(&self, reactor: Reactor) -> Self {
        Self(self.0.with_reactor(reactor))
    }
}

impl<U, W> Solve for Solver<U, W>
where
    U: Solve<Task = W::Task, Load = W::Load>,
    W: Memory,
{
    type Load = W::Load;
    type Task = W::Task;
    fn solve(&self, task: W::Task) -> W::Load {
        self.0.solve(task)
    }
}

impl<U, W> Reader for Solver<U, W>
where
    U: Read,
{
    type Unit = U::Unit;
    fn reader<F: FnOnce(&Self::Unit)>(&self, read: F) {
        self.0.reader(read);
    }
}

impl<U, W> Writer for Solver<U, W>
where
    U: Write,
    W: Clear,
{
    type Unit = U::Unit;
    fn writer<F: FnOnce(&mut Self::Unit)>(&self, write: F) {
        self.0.writer(write);
    }
}

impl<U, W> AddStem for Solver<U, W> {
    type Unit = U;
    fn add_stem<T, F: FnOnce(&mut U, T)>(&mut self, stem: T, add_stem: F) {
        self.0.add_stem(stem, add_stem);
    }
}

impl<U, W> AddReactor for Solver<U, W> {
    fn add_reactor(&mut self, reactor: &Reactor) {
        self.0.add_reactor(reactor);
    }
}

impl<U, W> React for Solver<U, W> {
    fn clear(&mut self) -> Reactors {
        self.0.clear()
    }
    fn react(&mut self) {
        self.0.react();
    }
}

// impl<N, W> AddStem for Solver<N, W>
// where
//     N: AddStem + React + 'static,
//     W: Clear + 'static,
//     N::Stem: WithReactor,
// {
//     type Stem = N::Stem;
//     fn add_stem(&mut self, stem: N::Stem) {
//         self.0.add_stem(stem);
//     }
// }

// impl<N, W> AddStem for Solver<N, W>
// where
//     U: AddStem<Stem = S> + React + 'static,
//     T: 'static,
//     L: 'static,
//     S: FromReactor + 'static,
// {
//     type Stem = U::Stem;
//     fn add_stem(&mut self, stem: U::Stem) {
//         self.0.add_stem(stem);
//     }
// }
