use crate::*;

pub struct Solver<U, W> {
    unit: U,
    work: W,
    reactors: Reactors, 
}

impl<U, W> FromUnit for Solver<U, W>
where
    W: Default,
{
    type Unit = U;
    fn new(unit: Self::Unit) -> Self {
        Self {
            unit,
            work: W::default(),
            reactors: Reactors::default(),
        }
    }
}

impl<U, W> SolveMut for Solver<U, W>
where
    W: Memory,
    U: Solve<Task = W::Task, Load = W::Load>,
{
    type Task = W::Task;
    type Load = W::Load;
    fn solve_mut(&mut self, task: W::Task) -> W::Load {
        if let Some(load) = self.work.get(&task) {
            load.clone()
        } else {
            let load = self.unit.solve(task.clone());
            self.work.add(task, load.clone());
            load
        }
    }
}

impl<U, W> React for Solver<U, W>
where
    U: React,
    W: Clear,
{
    fn clear(&mut self) -> Reactors {
        self.work.clear();
        self.reactors.clear()
    }
    fn react(&mut self) {
        self.unit.react();
    }
}

impl<N, W> AddReactor for Solver<N, W> {
    fn add_reactor(&mut self, reactor: Reactor) {
        self.reactors.add(reactor);
    }
}
