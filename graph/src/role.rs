use super::*;

/// Role of `link::Ploy` form.
pub type Ploy<P, L> = Role<P, link::Ploy<L>>;

/// Role of `link::Plan` form.
pub type Plan<P, T, L> = Role<P, link::Plan<T, L>>;

/// Two copies of the same link.
/// For unit access, the part field should be an enumeration of concrete links.
/// The form field should be link::Ploy or link::Plan to be used without unit knowledge.
/// Common use-case: Traverse and edit units or evaluate the load by way of anonymous units.  
pub struct Role<P, F> {
    pub part: P,
    pub form: F,
}

impl<P, F> Clone for Role<P, F>
where
    P: Clone,
    F: Clone,
{
    fn clone(&self) -> Self {
        Self {
            part: self.part.clone(),
            form: self.form.clone(),
        }
    }
}

impl<P, F> Grant for Role<P, F>
where
    F: Grant,
{
    type Load = F::Load;
    fn grant(&self) -> Self::Load {
        self.form.grant()
    }
}

impl<P, F> Solve for Role<P, F>
where
    F: Solve,
{
    type Task = F::Task;
    type Load = F::Load;
    fn solve(&self, task: Self::Task) -> Self::Load {
        self.form.solve(task)
    }
}

impl<P, F> Serve for Role<P, F>
where
    F: Serve,
{
    type Task = F::Task;
    type Load = F::Load;
    fn serve(&self, task: Self::Task) -> Self::Load {
        self.form.serve(task)
    }
}

impl<P, F> Backed for Role<P, F>
where
    P: Clone,
    F: Backed,
{
    fn backed(&self, back: &Back) -> Self {
        Self {
            part: self.part.clone(),
            form: self.form.backed(back),
        }
    }
}

impl<P, L> Serialize for Role<P, L>
where
    P: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.part.serialize(serializer)
    }
}
