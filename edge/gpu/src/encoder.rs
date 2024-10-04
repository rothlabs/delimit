pub use render::*;

use std::ops::Range;
use super::*;

mod render;

pub struct Encoder<'a> {
    pub inner: CommandEncoder,
    pub queue: &'a Queue,
}

impl<'a> Encoder<'a> {
    pub fn compute(&mut self) -> Compute<'_> {
        let inner = self.inner.begin_compute_pass(&ComputePassDescriptor {
            label: None,
            timestamp_writes: None,
        });
        Compute(inner)
    }
    pub fn render(&mut self, descriptor: &RenderPassDescriptor) -> Render<'_> {
        let inner = self.inner.begin_render_pass(descriptor);
        Render::new(inner)
    }
    pub fn copy_buffer(self, buffer: &'a Buffer) -> SourceBuffer<'a> {
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

pub struct Compute<'a>(
    ComputePass<'a>, // encoder: Encoder<'a>,
);

impl<'a> Compute<'a> {
    pub fn pipeline(mut self, pipeline: &ComputePipeline) -> Self {
        self.0.set_pipeline(pipeline);
        self
    }
    pub fn bind_group(mut self, index: u32, bind_group: &wgpu::BindGroup, offsets: &[u32]) -> Self {
        self.0.set_bind_group(index, bind_group, offsets);
        self
    }
    pub fn debug(mut self, label: &str) -> Self {
        self.0.insert_debug_marker(label);
        self
    }
    pub fn dispatch(mut self, x: u32, y: u32, z: u32) -> Self {
        self.0.dispatch_workgroups(x, y, z);
        self
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

// pub fn copy_buffer_to_buffer(mut self, source: &Buffer, source_offset: BufferAddress, destination: &Buffer, destination_offset: BufferAddress, copy_size: BufferAddress) -> Self {
//     self.inner.copy_buffer_to_buffer(source, source_offset, destination, destination_offset, copy_size);
//     self
// }
