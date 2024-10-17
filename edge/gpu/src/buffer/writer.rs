use std::fmt::Debug;
use super::*;

#[derive(Builder, Debug, Gate)] // , Output!
#[builder(pattern = "owned", setter(into))]
pub struct BufferWriter<T> { // : Payload + Pod
    queue: Grc<wgpu::Queue>,
    buffer: Hub<Grc<Buffer>>,
    #[builder(default)]
    offset: Hub<u64>,
    data: Hub<Vec<T>>,
}

impl<T> Act for BufferWriter<T>
where
    T: Pod + Debug,
{
    async fn act(&self) -> graph::Result<()> {
        let buffer = self.buffer.base().await?;
        let offset = self.offset.base().await.unwrap_or_default();
        self.data
            .read(|data| {
                self.queue
                    .write_buffer(&buffer, offset, cast_slice(data));
            })
            .await
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
