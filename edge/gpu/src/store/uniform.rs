use std::num::NonZero;

use super::*;

#[derive(Debug)]
pub struct UniformStore {
    pub layout: BindGroupLayout,
    pub group: Leaf<Grc<BindGroup>>,
    pub buffer: Leaf<Grc<Buffer>>,
    chunks: Leaf<Vec<Chunk>>,
}

impl UniformStore {
    pub fn new(device: &Device) -> Self {
        let buffer = uniform_buffer(device);
        let layout = uniform_layout(device);
        let group = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: BindingResource::Buffer(BufferBinding {
                    buffer: &buffer,
                    offset: 0,
                    size: Some(NonZero::new(256).unwrap())
                })
            }],
        });
        Self {
            layout,
            group: Leaf::new(Grc::new(group)),
            buffer: Leaf::new(buffer),
            chunks: Leaf::default(),
        }
    }
    // TODO: accept number of blocks of 64 elements
    pub fn grant(&self) -> Result<Leaf<u32>> {
        let max = (self.buffer.base()?.size() / 4) as u32;
        println!("uniform");
        grant(&self.chunks, 64, max)
    }
}
