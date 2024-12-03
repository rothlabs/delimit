use std::num::NonZero;

use super::*;

#[derive(Debug)]
pub struct UniformStore {
    pub layout: BindGroupLayout,
    pub layout_vertex: BindGroupLayout,
    pub group: Leaf<Grc<BindGroup>>,
    pub group_vertex: Leaf<Grc<BindGroup>>,
    pub buffer: Leaf<Grc<Buffer>>,
    chunks: Leaf<Vec<Chunk>>,
}

impl UniformStore {
    pub fn new(device: &Device) -> Self {
        let buffer = uniform_buffer(device);
        let layout = uniform_layout(device);
        let layout_vertex = uniform_layout_vertex(device);
        let entry = BindGroupEntry {
            binding: 0,
            resource: BindingResource::Buffer(BufferBinding {
                buffer: &buffer,
                offset: 0,
                size: Some(NonZero::new(256).unwrap()),
            }),
        };
        let group = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &layout,
            entries: &[entry.clone()],
        });
        let group_vertex = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &layout_vertex,
            entries: &[entry],
        });
        Self {
            layout,
            layout_vertex,
            group: Leaf::new(Grc::new(group)),
            group_vertex: Leaf::new(Grc::new(group_vertex)),
            buffer: Leaf::new(buffer),
            chunks: Leaf::default(),
        }
    }
    // TODO: accept number of blocks of 64 elements
    pub fn grant(&self) -> Result<Leaf<u32>> {
        let max = (self.buffer.base()?.size() / 4) as u32;
        grant(&self.chunks, 64, max)
    }
}
