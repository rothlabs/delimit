pub use compute::CommandBuilder;
pub use render::RenderBuilder;
pub use render::*;

use super::*;

pub mod post;
pub mod render;

mod compute;

pub struct Encode<'a> {
    pub inner: CommandEncoder,
    pub queue: &'a Queue,
}

impl<'a> Encode<'a> {
    pub fn compute(&mut self) -> ComputePass {
        self.inner
            .begin_compute_pass(&ComputePassDescriptor::default())
    }
    pub fn render(&mut self, descriptor: &RenderPassDescriptor) -> RenderPass {
        self.inner.begin_render_pass(descriptor)
    }
    pub fn copy_buffer(self, buffer: &'a Buffer) -> SourceBuffer<'_> {
        SourceBuffer {
            encoder: self,
            buffer,
            offset: 0,
        }
    }
    pub fn submit(self) -> SubmissionIndex {
        self.queue.submit(Some(self.inner.finish()))
    }
}

pub struct SourceBuffer<'a> {
    encoder: Encode<'a>,
    buffer: &'a Buffer,
    offset: BufferAddress,
}

impl<'a> SourceBuffer<'a> {
    pub fn offset(mut self, offset: BufferAddress) -> Self {
        self.offset = offset;
        self
    }
    pub fn destination(self, dest: &'a Buffer) -> DestinationBuffer<'a> {
        DestinationBuffer {
            source: self,
            destination: dest,
            offset: 0,
        }
    }
}

pub struct DestinationBuffer<'a> {
    source: SourceBuffer<'a>,
    destination: &'a Buffer,
    offset: BufferAddress,
}

impl<'a> DestinationBuffer<'a> {
    pub fn offset(mut self, offset: BufferAddress) -> Self {
        self.offset = offset;
        self
    }
    pub fn size(mut self, size: BufferAddress) -> Encode<'a> {
        self.source.encoder.inner.copy_buffer_to_buffer(
            self.source.buffer,
            self.source.offset,
            self.destination,
            self.offset,
            size,
        );
        self.source.encoder
    }
}
