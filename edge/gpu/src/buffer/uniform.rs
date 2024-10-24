use super::*;

#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "graph::Error"))]
#[builder(setter(into))]
pub struct Uniform<T> {
    gpu: Gpu,
    #[builder(setter(each(name = "field", into)))]
    fields: Vec<Hub<T>>,
}

impl<T> UniformBuilder<T> 
where 
    T: Pod + Debug
{
    pub async fn make(self) -> graph::Result<Hedge> {
        let build = self.build()?;
        // let size = build.fields.len() as u64 * 4;
        // let buffer = build.gpu.buffer(size).uniform()?;
        let vector = VectorBuilder::default().fields(build.fields).hub()?;
        build.gpu.hedge(vector, build.fields.len() as u64 * 4)
        // let root = build.gpu.writer(buffer.clone()).data(vector).hub()?;
        // Ok(Hedge { buffer: buffer.into(), root })
    }
}

// // TODO: make uniform produce a Hedge. The buffer size is constant but the values could change
// impl<T> Solve for Uniform<T>
// where
//     T: Pod + Debug,
// {
//     type Base = Grc<Buffer>;
//     async fn solve(&self) -> graph::Result<Hub<Grc<Buffer>>> {
//         // let vector = VectorBuilder::default();

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
