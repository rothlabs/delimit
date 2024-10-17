pub use reader::*;
pub use writer::*;

use super::*;
use std::{fmt::Debug, ops::Deref};

mod reader;
mod writer;

/// TODO: make separate BufferView that holds this Buffer and Mutators 
#[derive(Clone, Debug)]
pub struct Buffer {
    pub inner: Grc<wgpu::Buffer>,
    pub queue: Grc<wgpu::Queue>,
    pub stems: Vec<Hub<()>>,
}

impl Buffer {
    pub fn inner(&self) -> Grc<wgpu::Buffer> {
        self.inner.clone()
    }
    pub fn resource(&self) -> BindingResource {
        self.inner.as_entire_binding()
    }
    pub fn writer<T>(&self, data: impl Into<Hub<Vec<T>>>) -> BufferWriterBuilder<T> { // : Payload + Pod
        BufferWriterBuilder::default()
            .queue(self.queue.clone())
            .buffer(self.inner.clone())
            .data(data)
    }
    pub fn reader<T>(&self) -> BufferReaderBuilder<T> 
    // where 
    //     Vec<T>: Pod
    {
        BufferReaderBuilder::default().buffer(self.clone())
    }
}

impl Deref for Buffer {
    type Target = wgpu::Buffer;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "crate::Error"))]
pub struct BufferSetup<'a> {
    device: &'a Device,
    queue: Grc<wgpu::Queue>,
    #[builder(default, setter(strip_option))]
    label: Option<&'a str>,
    size: u64,
    usage: BufferUsages,
    #[builder(default)]
    mapped_at_creation: bool,
}

impl BufferSetupBuilder<'_> {
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
            stems: vec![]
        })
    }
    pub fn map_read(self) -> Result<Buffer> {
        self.usage(BufferUsages::MAP_READ | BufferUsages::COPY_DST)
            .make()
    }
    pub fn storage_copy(self) -> Result<Buffer> {
        self.usage(BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST)
            .make()
    }
}
