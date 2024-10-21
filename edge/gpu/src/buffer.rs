pub use reader::*;
pub use writer::*;

use super::*;

mod reader;
mod writer;

#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "graph::Error"))]
pub struct BufferSetup<'a> {
    device: &'a Device,
    #[builder(default, setter(strip_option))]
    label: Option<&'a str>,
    size: u64,
    usage: BufferUsages,
    #[builder(default)]
    mapped_at_creation: bool,
}

impl BufferSetupBuilder<'_> {
    pub fn make(self) -> graph::Result<Grc<Buffer>> {
        let built = self.build()?;
        let descriptor = BufferDescriptor {
            label: built.label,
            size: built.size,
            usage: built.usage,
            mapped_at_creation: built.mapped_at_creation,
        };
        let buffer = built.device.create_buffer(&descriptor);
        Ok(buffer.into())
    }
    pub fn map_read(self) -> graph::Result<Grc<Buffer>> {
        self.usage(BufferUsages::MAP_READ | BufferUsages::COPY_DST)
            .make()
    }
    pub fn storage_copy(self) -> graph::Result<Grc<Buffer>> {
        self.usage(BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST)
            .make()
    }
}

// Ok(Buffer {
//     inner: buffer.into(),
//     queue: built.queue,
//     // mutator: None
// })

// /// TODO: make separate BufferView that holds this Buffer and Mutators
// #[derive(Clone, Debug)]
// pub struct Buffer {
//     pub inner: Grc<wgpu::Buffer>,
//     pub queue: Grc<wgpu::Queue>,
//     // pub mutator: Option<Hub<Mutation>>,
// }

// impl Buffer {
//     // pub async fn depend(&self) -> graph::Result<Mutation> {
//     //     if let Some(mutator) = &self.mutator {
//     //         mutator.base().await
//     //     } else {
//     //         Ok(Mutation)
//     //     }
//     // }
//     pub fn inner(&self) -> Grc<wgpu::Buffer> {
//         self.inner.clone()
//     }
//     // pub fn inner(&self) -> Grc<wgpu::Buffer> {
//     //     self.inner.clone()
//     // }
//     pub fn resource(&self) -> BindingResource {
//         self.inner.as_entire_binding()
//     }
//     pub fn writer<T>(&self, data: impl Into<Hub<Vec<T>>>) -> BufferWriterBuilder<T> {
//         BufferWriterBuilder::default()
//             .queue(self.queue.clone())
//             .buffer(self.clone())
//             .data(data)
//     }
//     pub fn reader<T>(&self) -> BufferReaderBuilder<T> {
//         BufferReaderBuilder::default().buffer(self.clone())
//     }
// }

// impl Deref for Buffer {
//     type Target = wgpu::Buffer;
//     fn deref(&self) -> &Self::Target {
//         &self.inner
//     }
// }
