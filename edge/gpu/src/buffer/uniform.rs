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
    T: Pod + Debug,
{
    pub fn make(self) -> graph::Result<Hedge> {
        let build = self.build()?;
        let size = build.fields.len() as u64 * 4;
        let buffer: Hub<Grc<Buffer>> = build.gpu.buffer(size).uniform()?.into();
        let vector = VectorBuilder::default().fields(build.fields).hub()?;
        let root = build.gpu.writer(buffer.clone()).data(vector).hub()?;
        Ok(Hedge {
            buffer,
            root,
        })
    }
}