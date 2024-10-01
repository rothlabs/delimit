use super::*;

#[derive(Builder, Debug, Unit!)]
#[builder(pattern = "owned", setter(into))]
pub struct BufferWriter {
    queue: Grc<wgpu::Queue>,
    buffer: Grc<wgpu::Buffer>,
    #[builder(default)]
    offset: Hub<u64>,
    data: Hub<Vf32>,
}

impl Act for BufferWriter {
    async fn act(&self) -> graph::Result<()> {
        let offset = self.offset.base().await?;
        self.data.read(|data| {
            // TODO: check if this actually gets the data inside Vf32
            let slice = bytemuck::cast_slice(data);
            self.queue.write_buffer(&self.buffer, offset, slice);
        }).await
    }
    fn backed(&mut self, back: &Back) -> graph::Result<()> {
        self.offset.back(back)?;
        self.data.back(back)
    }
}