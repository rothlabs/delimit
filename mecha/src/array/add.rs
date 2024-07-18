use super::*;
use std::ops;

/// Add vector to each vector in base.
pub struct Add<N> {
    base: Stem<N>,
    vector: Array1<N>,
}

impl<N> Add<N>
where
    N: Copy + Default + ops::Add<N, Output = N> + Send + Sync + 'static,
{
    pub fn new(base: &Stem<N>, vector: &Array1<N>) -> Hold<Link<Self, N>, Role<N>> {
        let link = Link::new(Self {
            base: base.clone(),
            vector: vector.clone(),
        });
        let role = Role {
            part: Part::Add(link.clone()),
            form: link.plan(),
        };
        Hold { link, role }
    }
    pub fn array(&self) -> Array3<N> {
        let mut array = self.base.load(Task::Array).array();
        array.each(|i, b| self.vector.get([i[0]]) + b);
        array
    }
}

impl<N> Solve for Add<N>
where
    N: Copy + Default + ops::Add<N, Output = N> + Send + Sync + 'static,
{
    type Load = Load<N>;
    type Task = Task;
    fn solve(&self, task: Task) -> Load<N> {
        match task {
            Task::Array => Bare::Array(self.array()),
            Task::GpuRun => Bare::GpuRun,
        }
        .ace()
    }
}
