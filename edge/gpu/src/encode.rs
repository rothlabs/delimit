// pub use compute::ComputeBuilder;
pub use render::RenderBuilder;
pub use render::*;

use super::*;

pub mod render;

// mod compute;

pub struct Encode<'a> {
    pub inner: CommandEncoder,
    pub queue: &'a Queue,
}

impl<'a> Encode<'a> {
    pub fn render(&mut self, render: &flat::render::Pass, fields: &RenderPassDescriptor) {
        let mut pass = self.inner.begin_render_pass(fields);
        for step in render.steps.iter().rev() {
            match step {
                flat::render::Step::Pipe(pipe) => pass.set_pipeline(pipe),
                flat::render::Step::Bind(bind) => {
                    pass.set_bind_group(bind.slot, &bind.group, &bind.offsets)
                }
                flat::render::Step::Vertex(vertex) => {
                    pass.set_vertex_buffer(vertex.slot, vertex.buffer.slice(..));
                }
                flat::render::Step::Index(buffer) => {
                    pass.set_index_buffer(buffer.slice(..), IndexFormat::Uint16);
                }
                flat::render::Step::Draw(draw) => {
                    pass.draw(draw.vertices.clone(), draw.instances.clone());
                }
                flat::render::Step::DrawIndexed(indices, base_vertex, instances) => {
                    pass.draw_indexed(indices.clone(), *base_vertex, instances.clone());
                }
            }
        }
    }
    pub fn compute(&mut self, compute: &flat::compute::Pass) {
        let mut pass = self
            .inner
            .begin_compute_pass(&ComputePassDescriptor::default());
        for step in compute.steps.iter().rev() {
            match step {
                flat::compute::Step::Pipe(pipe) => pass.set_pipeline(pipe),
                flat::compute::Step::Bind(bind) => {
                    pass.set_bind_group(bind.slot, &bind.group, &bind.offsets)
                }
                flat::compute::Step::Dispatch(size) => {
                    println!("dispatch size: {size}");
                    pass.dispatch_workgroups(*size, 1, 1);
                },
            }
        }
    }
    pub fn copy_buffer(self, buffer: &'a Buffer) -> SourceBuffer<'_> {
        SourceBuffer {
            encoder: self,
            buffer,
            offset: 0,
        }
    }
    pub fn finish(self) -> CommandBuffer {
        self.inner.finish()
    }
    pub fn submit(self) -> SubmissionIndex {
        // let cb = self.inner.finish();
        self.queue.submit([self.finish()])
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

// pub fn compute(&mut self) -> ComputePass {
//     self.inner
//         .begin_compute_pass(&ComputePassDescriptor::default())
// }
// pub fn render(&mut self, descriptor: &RenderPassDescriptor) -> RenderPass {
//     self.inner.begin_render_pass(descriptor)
// }
