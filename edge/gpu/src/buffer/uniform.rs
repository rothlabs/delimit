use super::*;
use std::fmt::Debug;

#[derive(Builder, Gate, Debug)]
#[builder(pattern = "owned")]
#[builder(setter(into))]
pub struct Uniform<T> {
    gpu: Gpu,
    #[builder(setter(each(name = "field", into)))]
    fields: Vec<Hub<T>>,
}

impl<T> Solve for Uniform<T>
where
    T: Pod + Debug,
{
    type Base = Grc<Buffer>;
    async fn solve(&self) -> graph::Result<Hub<Grc<Buffer>>> {
        let mut data = vec![];
        for unit in &self.fields {
            data.push(unit.base().await?);
        }
        let buffer = self
            .gpu
            .buffer_init(&data, BufferUsages::UNIFORM | BufferUsages::COPY_DST)
            .into();
        Ok(buffer)
    }
}

impl<T> Adapt for Uniform<T>
where
    T: 'static + Clone,
{
    fn back(&mut self, back: &Back) -> graph::Result<()> {
        self.fields.back(back)
    }
}
