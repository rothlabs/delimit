type Work = graph::Work<Task, Load>;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub enum Task {
    #[default]
    String,
    Leaf,
}

#[derive(Clone)] // EnumAsInner
pub enum Load {
    String(String),
    Leaf(Leaf<String>),
}

impl Default for Load {
    fn default() -> Self {
        Load::String(String::new())
    }
}

impl<T> SolveReact<Task, Load> for Text<T>
where
    T: Solve<Load = Load, Task = Task> + 'static
{}

impl<T> Solve for Text<T>
where
    T: Solve<Load = Load, Task = Task> + 'static,
{
    type Load = Load;
    type Task = Task;
    fn solve(&self, task: Self::Task) -> Self::Load {
        self.0.solve(task)
    }
}

impl<T> SolverWithReactor for Text<T>
where
    T: Solve<Load = Load, Task = Task> + 'static,
{
    type Load = Load;
    type Task = Task;
    fn solver_with_reactor(
            &self,
            reactor: Reactor,
        ) -> Box<dyn SolveReact<Self::Task, Self::Load>> {
        self.0.solver_with_reactor(reactor)
    }
}

/////////////////////////////////////////////////////

pub struct TextSolver(Box<dyn SolveReact<Task, Load>>);

impl SolveReact<Task, Load> for TextSolver {}

impl Solve for TextSolver {
    type Load = Load;
    type Task = Task;
    fn solve(&self, task: Self::Task) -> Self::Load {
        self.0.solve(task)
    }
}

impl SolverWithReactor for TextSolver {
    type Load = Load;
    type Task = Task;
    fn solver_with_reactor(
            &self,
            reactor: Reactor,
        ) -> Box<dyn SolveReact<Self::Task, Self::Load>> {
        self.0.solver_with_reactor(reactor)
    }
}