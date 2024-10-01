use super::*;

// Unit!(BufferWriter<'_>);

#[derive(Builder, Debug)]
    #[builder(pattern = "owned", build_fn(error = "crate::Error"))]
pub struct BufferWriter<'a> {
    queue: &'a wgpu::Queue,
    buffer: &'a wgpu::Buffer,
    #[builder(default)]
    offset: Hub<u64>,
    array: Hub<Vf32>,
}

impl BufferWriterBuilder<'static> {
    pub fn make(self) -> Result<Node<BufferWriter<'static>>> {
        let wow = self.build()?; //.node()
        let node = wow.node()?;
        Ok(node)
    }
}

impl Act for BufferWriter<'_> {
    fn backed(&mut self, back: &Back) -> graph::Result<()> {
        self.offset.back(back)?;
        self.array.back(back)
    }
    async fn act(&self) -> graph::Result<()> {
        let offset = self.offset.base().await?;
        self.array.read(|array| {
            let slice = bytemuck::cast_slice(array);
            self.queue.write_buffer(self.buffer, offset, slice);
        }).await
    }
}