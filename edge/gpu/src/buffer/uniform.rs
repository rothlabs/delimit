use super::*;

#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "crate::Error"))]
#[builder(setter(into))]
pub struct Uniform<T> {
    core: Core,
    #[builder(setter(each(name = "field", into)))]
    fields: Vec<Hub<T>>,
}

impl<T> UniformBuilder<T>
where
    T: Pod + Debug + graph::SendSync,
{
    pub fn make(self) -> Result<BufferHedge> {
        let build = self.build()?;
        let size = build.fields.len() as u64 * 4;
        let buffer: Hub<Grc<Buffer>> = build.core.buffer(size).uniform()?.into();
        let vector = VectorBuilder::default().fields(build.fields).hub()?;
        let stem = build.core.writer(&buffer).data(vector).hub()?;
        bufferhedge().buffer(buffer).stem(stem).build()
        // Ok(Hedge { buffer, stem })
    }
}
