use super::*;

pub trait IntoRole {
    type Load;
    fn into_role(load: Self::Load) -> Self;
}

pub struct SolveRole<L, E> {
    pub exact: E,
    pub solver: Solver<L>,
}

impl<L, E> Clone for SolveRole<L, E>
where
    E: Clone,
{
    fn clone(&self) -> Self {
        Self {
            exact: self.exact.clone(),
            solver: self.solver.clone(),
        }
    }
}

impl<L, E> Grant for SolveRole<L, E> {
    type Load = L;
    fn grant(&self) -> Self::Load {
        self.solver.grant()
    }
}

impl<L, E> WithRoot for SolveRole<L, E>
where
    E: Clone,
{
    type Root = Root;
    fn with_root(&self, root: &Self::Root) -> Self {
        Self {
            exact: self.exact.clone(),
            solver: self.solver.with_root(root),
        }
    }
}

pub struct TaskRole<T, L, E> {
    pub exact: E,
    pub tasker: Tasker<T, L>,
}

impl<T, L, E> Clone for TaskRole<T, L, E>
where
    E: Clone,
{
    fn clone(&self) -> Self {
        Self {
            exact: self.exact.clone(),
            tasker: self.tasker.clone(),
        }
    }
}

impl<T, L, E> SolveTask for TaskRole<T, L, E> {
    type Task = T;
    type Load = L;
    fn solve_task(&self, task: T) -> L {
        self.tasker.solve_task(task)
    }
}

impl<T, L, E> WithRoot for TaskRole<T, L, E>
where
    E: Clone,
{
    type Root = Root;
    fn with_root(&self, root: &Self::Root) -> Self {
        Self {
            exact: self.exact.clone(),
            tasker: self.tasker.with_root(root),
        }
    }
}