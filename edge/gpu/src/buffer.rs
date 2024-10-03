use std::{marker::PhantomData, ops::Deref};

use super::*;

#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "crate::Error"))]
pub struct BufferRubric<'a> {
    device: &'a Device,
    queue: Grc<wgpu::Queue>,
    #[builder(default, setter(strip_option))]
    label: Option<&'a str>,
    size: u64,
    usage: BufferUsages,
    #[builder(default)]
    mapped_at_creation: bool,
}

impl BufferRubricBuilder<'_> {
    pub fn make(self) -> Result<Buffer> {
        let built = self.build()?;
        let descriptor = BufferDescriptor {
            label: built.label,
            size: built.size,
            usage: built.usage,
            mapped_at_creation: built.mapped_at_creation,
        };
        let buffer = built.device.create_buffer(&descriptor);
        Ok(Buffer {
            inner: buffer.into(),
            queue: built.queue,
        })
    }
    pub fn map_read(self) -> Self {
        self.usage(BufferUsages::MAP_READ | BufferUsages::COPY_DST)
    }
}

#[derive(Clone)]
pub struct Buffer {
    inner: Grc<wgpu::Buffer>,
    queue: Grc<wgpu::Queue>,
}

impl Buffer {
    pub fn resource(&self) -> BindingResource {
        self.inner.as_entire_binding()
    }
    pub fn writer<T: Payload>(&self) -> BufferWriterBuilder<T> {
        BufferWriterBuilder::default()
            .queue(self.queue.clone())
            .buffer(self.inner.clone())
    }
    pub async fn read<T: bytemuck::Pod>(&self) -> Result<Vec<T>> {
        let buffer_slice = self.inner.slice(..);
        let (sender, receiver) = flume::bounded(1);
        buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
        receiver.recv_async().await??;
        let data = buffer_slice.get_mapped_range();
        Ok(bytemuck::cast_slice(&data).to_vec())
    } 
}

impl Deref for Buffer {
    type Target = wgpu::Buffer;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Builder, Debug, UnitGp!)]
#[builder(pattern = "owned", setter(into))]
pub struct BufferWriter<T: Payload> {
    queue: Grc<wgpu::Queue>,
    buffer: Grc<wgpu::Buffer>,
    #[builder(default)]
    offset: Hub<u64>,
    data: Hub<T>,
}

impl<T> Act for BufferWriter<T> 
where 
    T: Payload + CastSlice
{
    async fn act(&self) -> graph::Result<()> {
        let offset = self.offset.base().await?;
        self.data
            .read(|data| {
                self.queue.write_buffer(&self.buffer, offset, data.slice());
            })
            .await
    }
    fn backed(&mut self, back: &Back) -> graph::Result<()> {
        self.offset.back(back)?;
        self.data.back(back)
    }
}

#[derive(Builder, Debug, UnitGpO!)]
#[builder(pattern = "owned", setter(into))]
pub struct BufferReader<T: Payload> {
    buffer: Grc<wgpu::Buffer>,
    phantom: PhantomData<T>,
}

impl<T> Solve for BufferReader<T> 
where 
    T: Payload + AnyBitPattern
{
    type Base = T;
    async fn solve(&self) -> graph::Result<Hub<T>> {
        let buffer_slice = self.buffer.slice(..);
        let (sender, receiver) = flume::bounded(1);
        buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
        let wow = receiver.recv_async().await?;
        let data = buffer_slice.get_mapped_range();
        let out: Vec<T> = bytemuck::cast_slice(&data).to_vec();
        Ok(bytemuck::cast_slice(&data).to_vec())
    }
    // fn backed(&mut self, back: &Back) -> graph::Result<()> {
    //     self.offset.back(back)?;
    //     self.data.back(back)
    // }
}

