use crate::*;

pub struct Solver<U, W>(Edge<node::Solver<U, W>>);

impl<U, W> FromUnit for Solver<U, W>
where
    //N: FromUnit,
    W: Default,
{
    type Unit = U; //N::Unit;
    fn from_unit(unit: Self::Unit) -> Self {
        Self(edge::Edge::from_unit(unit))
    }
}

impl<U, W> WithReactor for Solver<U, W> {
    fn with_reactor<T: ToReactor>(&self, item: T) -> Self {
        Self(self.0.with_reactor(item))
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
{
    type Unit = U::Unit;
    fn writer<F: FnOnce(&mut U::Unit)>(&self, write: F) {
        self.0.writer(write);
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
