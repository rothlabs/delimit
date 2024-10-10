use super::*;

#[derive(Builder, Debug, Output!)]
#[builder(pattern = "owned", setter(into))]
pub struct BufferWriter<T: Payload + Pod> {
    queue: Grc<wgpu::Queue>,
    buffer: Grc<wgpu::Buffer>,
    #[builder(default)]
    offset: Hub<u64>,
    data: Hub<Vec<T>>,
}

impl<T> Act for BufferWriter<T>
where
    T: Payload + Pod,
{
    async fn act(&self) -> graph::Result<()> {
        let offset = self.offset.base().await?;
        self.data
            .read(|data| {
                self.queue
                    .write_buffer(&self.buffer, offset, cast_slice(data));
            })
            .await
    }
}

impl<T> Adapt for BufferWriter<T> 
where
    T: Payload + Pod,
{
    fn back(&mut self, back: &Back) -> graph::Result<()> {
        self.offset.back(back)?;
        self.data.back(back)
    }
}