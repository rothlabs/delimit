use super::*;

pub trait IntoRole {
    type Load;
    fn into_role(load: Self::Load) -> Self;
}

pub struct PloyRole<L, E> {
    pub ploy: Ploy<L>,
    pub exact: E,
}

impl<L, E> Clone for PloyRole<L, E>
where
    E: Clone,
{
    fn clone(&self) -> Self {
        Self {
            exact: self.exact.clone(),
            ploy: self.ploy.clone(),
        }
    }
}

impl<L, E> Grant for PloyRole<L, E> {
    type Load = L;
    fn grant(&self) -> Self::Load {
        self.ploy.grant()
    }
}

impl<L, E> WithRoot for PloyRole<L, E>
where
    E: Clone,
{
    type Root = Back;
    fn with_root(&self, root: &Self::Root) -> Self {
        Self {
            exact: self.exact.clone(),
            ploy: self.ploy.with_root(root),
        }
    }
}

pub struct PlanRole<T, L, E> {
    pub exact: E,
    pub plan: Plan<T, L>,
}

impl<T, L, E> Clone for PlanRole<T, L, E>
where
    E: Clone,
{
    fn clone(&self) -> Self {
        Self {
            exact: self.exact.clone(),
            plan: self.plan.clone(),
        }
    }
}

impl<T, L, E> Solve for PlanRole<T, L, E> {
    type Task = T;
    type Load = L;
    fn solve(&self, task: T) -> L {
        self.plan.solve(task)
    }
}

impl<T, L, E> WithRoot for PlanRole<T, L, E>
where
    E: Clone,
{
    type Root = Back;
    fn with_root(&self, root: &Self::Root) -> Self {
        Self {
            exact: self.exact.clone(),
            plan: self.plan.with_root(root),
        }
    }
}