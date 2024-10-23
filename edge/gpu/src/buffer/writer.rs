use super::*;
use std::fmt::Debug;

#[derive(Builder, Gate, Debug)]
#[builder(pattern = "owned")]
#[builder(setter(into))]
pub struct BufferWriter<T> {
    queue: Grc<wgpu::Queue>,
    buffer: Hub<Grc<Buffer>>,
    #[builder(default)]
    offset: Hub<u64>,
    data: Hub<Vec<T>>,
}

impl<T> Solve for BufferWriter<T>
where
    T: Pod + Debug,
{
    type Base = Mutation;
    async fn solve(&self) -> graph::Result<Hub<Mutation>> {
        let buffer = self.buffer.base().await?;
        let offset = self.offset.base().await.unwrap_or_default();
        self.data
            .read(|data| {
                self.queue.write_buffer(&buffer, offset, cast_slice(data));
            })
            .await?;
        Ok(Mutation {}.into())
    }
}

impl<T> Adapt for BufferWriter<T>
where
    T: 'static + Clone,
{
    fn back(&mut self, back: &Back) -> graph::Result<()> {
        self.offset.back(back)?;
        self.data.back(back)
    }
}
