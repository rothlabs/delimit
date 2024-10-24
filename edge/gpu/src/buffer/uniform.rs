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

// TODO: make uniform produce a Hedge. The buffer size is constant but the values could change
impl<T> Solve for Uniform<T>
where
    T: Pod + Debug,
{
    type Base = Grc<Buffer>;
    async fn solve(&self) -> graph::Result<Hub<Grc<Buffer>>> {
        // let vector = VectorBuilder::default();

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

// #[derive(Builder, Gate, Debug)]
// #[builder(pattern = "owned")]
// #[builder(setter(into))]
// pub struct Uniform<T> {
//     gpu: Gpu,
//     #[builder(setter(each(name = "field", into)))]
//     fields: Vec<Hub<T>>,
// }

// // TODO: make uniform produce a Hedge. The buffer size is constant but the values could change
// impl<T> Solve for Uniform<T>
// where
//     T: Pod + Debug,
// {
//     type Base = Grc<Buffer>;
//     async fn solve(&self) -> graph::Result<Hub<Grc<Buffer>>> {
//         let mut data = vec![];
//         for unit in &self.fields {
//             data.push(unit.base().await?);
//         }
//         let buffer = self
//             .gpu
//             .buffer_init(&data, BufferUsages::UNIFORM | BufferUsages::COPY_DST)
//             .into();
//         Ok(buffer)
//     }
// }

// impl<T> Adapt for Uniform<T>
// where
//     T: 'static + Clone,
// {
//     fn back(&mut self, back: &Back) -> graph::Result<()> {
//         self.fields.back(back)
//     }
// }
