use crate::*;

pub struct UnitTasker<U, W> {
    unit: U,
    work: W,
    reactors: Reactors,
}

impl<U, W> FromUnit for UnitTasker<U, W>
where
    W: Default,
{
    type Unit = U;
    fn new(unit: Self::Unit) -> Self {
        Self {
            unit,
            work: W::default(),
            reactors: Reactors::new(),
        }
    }
}

impl<U, W> Read for UnitTasker<U, W> {
    type Unit = U;
    fn read(&self) -> &U {
        &self.unit
    }
}

impl<U, W> SolveTaskMut for UnitTasker<U, W>
where
    W: Memory,
    U: SolveTask<Task = W::Task, Load = W::Load>,
{
    type Task = W::Task;
    type Load = W::Load;
    fn solve_task_mut(&mut self, task: W::Task) -> W::Load {
        if let Some(load) = self.work.get(&task) {
            load.clone()
        } else {
            let load = self.unit.solve_task(task.clone());
            self.work.add(task, load.clone());
            load
        }
    }
}

impl<U, W> Write for UnitTasker<U, W>
where
    W: Clear,
{
    type Unit = U;
    fn write<F: FnOnce(&mut U)>(&mut self, write: F) {
        write(&mut self.unit);
        self.work.clear();
        self.reactors.cycle();
    }
}

impl<U, W> WriteWithReactor for UnitTasker<U, W>
where
    W: Clear,
{
    type Unit = U;
    fn write_with_reactor<F: FnOnce(&mut WriterPack<U>)>(
        &mut self,
        write: F,
        reactor: &Reactor,
    ) {
        // write(&mut self.unit, reactor);
        write(&mut WriterPack{unit: &mut self.unit, reactor: reactor});
        self.work.clear();
        self.reactors.cycle();
    }
}

impl<U, W> React for UnitTasker<U, W>
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

impl<U, W> AddStem for UnitTasker<U, W> {
    type Unit = U;
    fn add_stem<T, F: FnOnce(&mut U, T)>(&mut self, stem: T, add_stem: F) {
        add_stem(&mut self.unit, stem);
        self.reactors.cycle();
    }
}

impl<U, W> AddReactor for UnitTasker<U, W> {
    fn add_reactor(&mut self, reactor: Reactor) {
        self.reactors.add(reactor);
    }
}
