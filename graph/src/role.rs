use super::*;

pub trait IntoRole {
    type Load;
    fn into_role(load: Self::Load) -> Self;
}

pub struct Role<A, M> {
    pub actual: A,
    pub method: M,
}

impl<A, M> Clone for Role<A, M>
where
    A: Clone,
    M: Clone,
{
    fn clone(&self) -> Self {
        Self {
            actual: self.actual.clone(),
            method: self.method.clone(),
        }
    }
}

impl<A, M> Grant for Role<A, M> 
where 
    M: Grant
{
    type Load = M::Load;
    fn grant(&self) -> Self::Load {
        self.method.grant()
    }
}

impl<A, M> WithRoot for Role<A, M>
where
    A: WithRoot,
    M: Clone,
{
    type Root = A::Root;
    fn with_root(&self, root: &Self::Root) -> Self {
        Self {
            actual: self.actual.with_root(root),
            method: self.method.clone(),
        }
    }
}

// impl<L, E> WithRoot for Role<L, E>
// where
//     E: Clone,
// {
//     type Root = Back;
//     fn with_root(&self, root: &Self::Root) -> Self {
//         Self {
//             actual: self.actual.clone(),
//             method: self.method.with_root(root),
//         }
//     }
// }

// pub struct PlanRole<T, L, E> {
//     pub exact: E,
//     pub plan: Plan<T, L>,
// }

// impl<T, L, E> Clone for PlanRole<T, L, E>
// where
//     E: Clone,
// {
//     fn clone(&self) -> Self {
//         Self {
//             exact: self.exact.clone(),
//             plan: self.plan.clone(),
//         }
//     }
// }

// impl<T, L, E> Solve for PlanRole<T, L, E> {
//     type Task = T;
//     type Load = L;
//     fn solve(&self, task: T) -> L {
//         self.plan.solve(task)
//     }
// }

// impl<T, L, E> WithRoot for PlanRole<T, L, E>
// where
//     E: Clone,
// {
//     type Root = Back;
//     fn with_root(&self, root: &Self::Root) -> Self {
//         Self {
//             exact: self.exact.clone(),
//             plan: self.plan.with_root(root),
//         }
//     }
// }