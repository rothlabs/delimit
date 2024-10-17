use std::fmt::Debug;

use super::*;

#[derive(Builder, Debug)] // , Output!
#[builder(pattern = "owned", setter(into))]
pub struct BufferWriter<T> { // : Payload + Pod
    queue: Grc<wgpu::Queue>,
    buffer: Grc<wgpu::Buffer>,
    // buffer: Hub<graph::Buffer>,
    #[builder(default)]
    offset: Hub<u64>,
    data: Hub<Vec<T>>,
}

impl<T> GateTag for BufferWriter<T> {}

impl<T> BufferWriterBuilder<T>
where
    // T: 'static + std::fmt::Debug + Clone,
    T: 'static + Clone + Debug,
    BufferWriter<T>: Solve,
    <BufferWriter<T> as Solve>::Base: Clone + Debug,
{
    pub fn make(self) -> graph::Result<BufferWriter<T>> {
        match self.build() {
            Ok(value) => Ok(value),
            Err(err) => Err(anyhow!(err.to_string()))?,
        }
    }
    pub fn node(self) -> graph::Result<Node<BufferWriter<T>>> {
        self.make()?.node()
    }
    pub fn hub(self) -> graph::Result<Hub<<BufferWriter<T> as Solve>::Base>> {
        Ok(self.make()?.gate()?.into())
    }
}

impl<T> Act for BufferWriter<T>
where
    T: Pod + Debug, // T: Payload + Pod,
{
    async fn act(&self) -> graph::Result<()> {
        let offset = self.offset.base().await.unwrap_or_default();
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
    T: 'static + Clone, // T: Payload + Pod,
{
    fn back(&mut self, back: &Back) -> graph::Result<()> {
        self.offset.back(back)?;
        self.data.back(back)
    }
}
