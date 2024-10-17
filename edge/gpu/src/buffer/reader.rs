use super::*;

#[derive(Builder, Debug, Gate)]
#[builder(pattern = "owned")]
#[builder(setter(into))]
pub struct BufferReader<T> {
    mutator: Hub<Mutation>,
    buffer: Hub<Grc<Buffer>>,
    #[builder(default)]
    phantom: std::marker::PhantomData<T>,
}

impl<T> Solve for BufferReader<T>
where
    T: Pod,
{
    type Base = Vec<T>;
    async fn solve(&self) -> graph::Result<Hub<Vec<T>>> {
        self.mutator.base().await?;
        let buffer = self.buffer.base().await?;
        let slice = buffer.slice(..);
        let (sender, receiver) = flume::bounded(1);
        slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
        if let Err(err) = receiver.recv_async().await? {
            return Err(anyhow!(err))?;
        }
        let data = slice.get_mapped_range();
        let out = bytemuck::cast_slice(&data).to_vec();
        Ok(out.into_leaf().hub())
    }
}

impl<T> Adapt for BufferReader<T> {
    fn back(&mut self, back: &Back) -> graph::Result<()> {
        self.mutator.back(back)?;
        self.buffer.back(back)
    }
}