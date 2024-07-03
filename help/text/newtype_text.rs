pub struct Text<U>(UnitSolver<U, Leaf<String>>);

impl<U> Text<U>
where
    U: Solve<Load = Leaf<String>> + React + 'static,
{
    pub fn new(unit: U) -> Self {
        Self(UnitSolver::new(unit))
    }
    pub fn solve(&self) -> Leaf<String> {
        self.0.solve()
    }
    pub fn solver(&self) -> link::Solver<Leaf<String>> {
        self.0.solver()
    }
    pub fn writer<F: FnOnce(&mut U)>(&self, write: F) {
        self.0.writer(write);
    }
    pub fn writer_with_reactor<F: Fn(&mut U, &Reactor)>(&self, writer: F) {
        self.0.writer_with_reactor(writer);
    }
    pub fn stemmer<S: WithReactor, F: FnOnce(&mut U, S)>(&self, stem: &S, add_stem: F) {
        self.0.stemmer(stem, add_stem);
    }
}

impl<U> WithReactor for Text<U> {
    fn with_reactor(&self, reactor: &Reactor) -> Self {
        Self(self.0.with_reactor(reactor))
    }
}

impl<U> Clone for Text<U> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}