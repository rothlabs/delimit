use super::*;

#[derive(Debug, Gate)]
pub struct Group {
    pub device: Grc<Device>,
    pub layout: Grc<BindGroupLayout>,
    pub entries: Vec<GroupEntry>,
}

impl Solve for Group {
    type Base = Grc<wgpu::BindGroup>;
    async fn solve(&self) -> node::Result<Self::Base> {
        let mut buffers = vec![];
        for entry in &self.entries {
            buffers.push(entry.buffer.base().await?);
        }
        let mut entries = vec![];
        for (entry, buffer) in self.entries.iter().zip(&buffers) {
            entries.push(BindGroupEntry {
                binding: entry.slot,
                resource: BindingResource::Buffer(BufferBinding {
                    buffer,
                    offset: 0,
                    size: entry.size,
                }),
            });
        }
        let bind = self.device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &self.layout,
            entries: &entries,
        });
        Ok(Grc::new(bind).into())
    }
}

impl Adapt for Group {
    fn back(&mut self, back: &Back) {
        for entry in &mut self.entries {
            entry.buffer.back(back);
        }
    }
}

#[derive(Debug, Clone)]
pub struct GroupEntry {
    pub slot: u32,
    pub buffer: Hub<Grc<Buffer>>,
    pub size: Option<NonZero<u64>>,
}