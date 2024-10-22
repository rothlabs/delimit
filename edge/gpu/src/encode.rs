pub use compute::ComputeBuilder;
pub use render::RenderBuilder;
pub use render::*;

use super::*;

mod compute;
mod pass;
mod render;

pub struct Encoder<'a> {
    pub inner: CommandEncoder,
    pub queue: &'a Queue,
}

impl<'a> Encoder<'a> {
    pub fn compute(&mut self) -> ComputePass {// pass::Compute<'_> {
        let pass = self
            .inner
            .begin_compute_pass(&ComputePassDescriptor::default());
        pass
        // pass::Compute::new(pass)
    }
    pub fn render(&mut self, descriptor: &RenderPassDescriptor) -> pass::Render<'_> {
        let pass = self.inner.begin_render_pass(descriptor);
        pass::Render::new(pass)
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
    encoder: Encoder<'a>,
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
    pub fn size(mut self, size: BufferAddress) -> Encoder<'a> {
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
