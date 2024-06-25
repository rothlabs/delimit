use crate::*;

pub struct Solver<N, W>(Edge<node::Solver<N, W>>);

impl<N, W> FromUnit for Solver<N, W>
where  
    N: FromUnit,
    W: Default,
{
    type Unit = N::Unit;
    fn from_unit(unit: N::Unit) -> Self {
        Self(edge::Edge::from_unit(unit))
    }
}

impl<N, W> FromReactor for Solver<N, W> {
    fn from_reactor(&self, reactor: Reactor) -> Self {
        Self(self.0.from_reactor(reactor))
    }
}

impl<N, W> Solve for Solver<N, W>
where
    N: Solve<Task = W::Task, Load = W::Load>,
    W: Work,
{
    type Load = W::Load;
    type Task = W::Task;
    fn solve(&self, task: W::Task) -> W::Load {
        self.0.solve(task)
    }
}

impl<N, W> AddStem for Solver<N, W>
where
    N: AddStem + React + 'static,
    W: Clear + 'static,
    N::Stem: FromReactor,
{
    type Stem = N::Stem;
    fn add_stem(&mut self, stem: N::Stem) {
        self.0.add_stem(stem);
    }
}



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