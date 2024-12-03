use super::*;

#[derive(Debug)]
pub struct StorageStore {
    pub layout: BindGroupLayout,
    pub group: Leaf<Grc<BindGroup>>,
    pub buffer: Leaf<Grc<Buffer>>,
    chunks: Leaf<Vec<Chunk>>,
}

impl StorageStore {
    pub fn new(device: &Device) -> Self {
        let buffer = storage_buffer(device);
        let layout = storage_layout(device);
        let group = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: buffer.as_entire_binding(),
                },
            ],
        });
        Self {
            layout,
            group: Leaf::new(Grc::new(group)),
            buffer: Leaf::new(buffer),
            chunks: Leaf::default(),
        }
    }
    pub fn grant(&self, size: u32) -> Result<Leaf<u32>> {
        let max = (self.buffer.base()?.size() / 4) as u32;
        println!("storage");
        grant(&self.chunks, size, max)
    }
}
