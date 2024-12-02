use super::*;

#[derive(Debug)]
pub struct UniformStore {
    layout: BindGroupLayout,
    group: Leaf<Grc<BindGroup>>,
    buffer: Leaf<Grc<Buffer>>,
    chunks: Leaf<Vec<Chunk>>,
}

impl UniformStore {
    pub fn new(gpu: &Gpu) -> Self {
        let buffer = uniform_buffer(&gpu.device);
        let layout = uniform_layout(&gpu.device);
        let group = gpu.device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });
        Self {
            layout,
            group: Leaf::new(Grc::new(group)),
            buffer: Leaf::new(buffer),
            chunks: Leaf::default(),
        }
    }
    pub fn grant(&self, size: u32) -> Result<Leaf<u32>> {
        let mut i = 0;
        let mut start = 0;
        let mut end = size;
        let grant = self.chunks.write_passive(|chunks| {
            while let Some(chunk) = chunks.get(i) {
                if end < chunk.start {
                    break;
                } else {
                    start = chunk.end;
                    end = chunk.end + size;
                }
                i += 1;
            }
            let grant = Leaf::new(start);
            let chunk = Chunk {
                grant: grant.clone(),
                start,
                end,
            };
            chunks.insert(i, chunk);
            grant
        })?;
        Ok(grant)
    }
}