use super::*;
use std::ops::Range;

pub struct Compute<'a>(
    ComputePass<'a>, // encoder: Encoder<'a>,
);

impl<'a> Compute<'a> {
    pub fn new(pass: ComputePass<'a>) -> Self {
        Self(pass)
    }
    pub fn pipe(mut self, pipeline: &ComputePipeline) -> Self {
        self.0.set_pipeline(pipeline);
        self
    }
    pub fn bind(mut self, index: u32, bind_group: &BindGroup, offsets: &[u32]) -> Self {
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

pub struct Render<'a>(RenderPass<'a>);

impl<'a> Render<'a> {
    pub fn new(render_pass: RenderPass<'a>) -> Self {
        Self(render_pass)
    }
    pub fn pipe(mut self, pipeline: &RenderPipeline) -> Self {
        self.0.set_pipeline(pipeline);
        self
    }
    pub fn vertex(mut self, slot: u32, buffer_slice: BufferSlice) -> Self {
        self.0.set_vertex_buffer(slot, buffer_slice);
        self
    }
    pub fn debug(mut self, label: &str) -> Self {
        self.0.insert_debug_marker(label);
        self
    }
    pub fn draw(mut self, vertices: Range<u32>, instances: Range<u32>) -> Self {
        self.0.draw(vertices, instances);
        self
    }
}
