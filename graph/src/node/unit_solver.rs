use crate::*;

pub struct UnitSolver<U, L> {
    unit: U,
    load: Option<L>,
    reactors: Reactors,
}

impl<U, L> FromUnit for UnitSolver<U, L> {
    type Unit = U;
    fn new(unit: Self::Unit) -> Self {
        Self {
            unit,
            load: None,
            reactors: Reactors::new(),
        }
    }
}

impl<U, L> Read for UnitSolver<U, L> {
    type Unit = U;
    fn read(&self) -> &U {
        &self.unit
    }
}

impl<U, L> SolveMut for UnitSolver<U, L>
where
    U: Solve<Load = L>,
    L: Clone,
{
    type Load = L;
    fn solve_mut(&mut self) -> L {
        if let Some(load) = &self.load {
            load.clone()
        } else {
            let load = self.unit.solve();
            self.load = Some(load.clone());
            load
        }
    }
}

impl<U, L> Write for UnitSolver<U, L> {
    type Unit = U;
    fn write<F: FnOnce(&mut U)>(&mut self, write: F) {
        write(&mut self.unit);
        self.load = None;
        self.reactors.cycle();
    }
}

impl<U, L> WriteWithReactor for UnitSolver<U, L> {
    type Unit = U;
    fn write_with_reactor<F: FnOnce(&mut WriterPack<U>)>(&mut self, write: F, reactor: &Reactor) {
        write(&mut WriterPack {
            unit: &mut self.unit,
            reactor: reactor,
        });
        self.load = None;
        self.reactors.cycle();
    }
}

impl<U, L> React for UnitSolver<U, L>
// where
//     U: React,
{
    fn clear(&mut self) -> Reactors {
        self.load = None;
        // if these were Responders, it would need to take a default memo and still return reactors
        self.reactors.clear()
    }
    fn react(&mut self) {
        // self.unit.react();
    }
}

impl<U, L> AddStem for UnitSolver<U, L> {
    type Unit = U;
    fn add_stem<T, F: FnOnce(&mut U, T)>(&mut self, stem: T, add_stem: F) {
        add_stem(&mut self.unit, stem);
        self.reactors.cycle();
    }
}

impl<U, L> AddReactor for UnitSolver<U, L> {
    fn add_reactor(&mut self, reactor: Reactor) {
        self.reactors.add(reactor);
    }
}
