pub use compute::ComputeBuilder;
pub use render::RenderBuilder;
pub use render::*;

use super::*;

// pub mod post;
pub mod render;

mod compute;

pub struct Encode<'a> {
    pub inner: CommandEncoder,
    pub queue: &'a Queue,
}

impl<'a> Encode<'a> {
    pub fn render(&mut self, render: &crate::render::Pass, fields: &RenderPassDescriptor) {
        // match pass {
        //     Pass::Render(render) => {
        let mut pass = self.inner.begin_render_pass(fields);
        for entry in &render.steps {
            match entry {
                crate::render::Step::Pipe(pipe) => pass.set_pipeline(pipe),
                crate::render::Step::Bind(index, bind) => pass.set_bind_group(*index, bind, &[]),
                crate::render::Step::Vertex(slot, buffer) => {
                    pass.set_vertex_buffer(*slot, buffer.slice(..));
                }
                crate::render::Step::Index(buffer) => {
                    pass.set_index_buffer(buffer.slice(..), IndexFormat::Uint16);
                }
                crate::render::Step::Draw(vertices, instances) => {
                    pass.draw(vertices.clone(), instances.clone());
                }
                crate::render::Step::DrawIndexed(indices, base_vertex, instances) => {
                    pass.draw_indexed(indices.clone(), *base_vertex, instances.clone());
                }
            }
        }
        //     }
        //     _ => ()
        // }
    }
    pub fn compute(&mut self) -> ComputePass {
        self.inner
            .begin_compute_pass(&ComputePassDescriptor::default())
    }
    // pub fn render(&mut self, descriptor: &RenderPassDescriptor) -> RenderPass {
    //     self.inner.begin_render_pass(descriptor)
    // }
    pub fn copy_buffer(self, buffer: &'a Buffer) -> SourceBuffer<'_> {
        SourceBuffer {
            encoder: self,
            buffer,
            offset: 0,
        }
    }
    pub fn submit(self) -> SubmissionIndex {
        self.queue.submit([self.inner.finish()])
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
