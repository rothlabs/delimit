use std::num::NonZero;
use super::*;

#[derive(Debug)]
pub struct GroupUnit {
    pub device: Grc<Device>,
    pub layout: Grc<BindGroupLayout>,
    pub entries: Vec<GroupEntry>,
}

impl Solve for GroupUnit {
    type Base = Grc<wgpu::BindGroup>;
    async fn solve(&self) -> node::Result<Self::Base> {
        let mut buffers = vec![];
        for entry in &self.entries {
            buffers.push(entry.buffer.base().await?);
        }
        let mut entries = vec![];
        for (entry, buffer) in self.entries.iter().zip(&buffers) {
            entries.push(if let Some(size) = entry.size {
                BindGroupEntry {
                    binding: entry.slot,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer,
                        offset: 0,
                        size: Some(NonZero::new(size as u64).ok_or(anyhow!("size must be non-zero"))?),
                    })
                }
            } else {
                BindGroupEntry {
                    binding: entry.slot,
                    resource: buffer.as_entire_binding(),
                }
            })
        }
        let bind = self.device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &self.layout,
            entries: &entries,
        });
        Ok(Grc::new(bind).into())
    }
}

impl Adapt for GroupUnit {
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
    pub size: Option<u32>,
}



// pub struct BindGroupUnit {
//     device: Grc<Device>,
//     layout: Grc<BindGroupLayout>,
//     entries: Vec<BufferBind>,
// }

// impl Solve for BindGroupUnit {
//     type Base = Grc<BindGroup>;
//     async fn solve(&self) -> node::Result<Self::Base> {
//         let mut buffers = vec![];
//         for (binding, buffer) in &self.entries {
//             buffers.push((*binding, buffer.base().await?));
//         }
//         let mut entries = vec![];
//         for (i, buffer) in &buffers {
//             //let wow = buffer.as_entire_buffer_binding();
//             let resource = buffer.as_entire_binding();
//             entries.push(BindGroupEntry {
//                 binding: *i,
//                 resource,
//             });
//         }
//         let bind = self.device.create_bind_group(&BindGroupDescriptor {
//             label: None,
//             layout: &self.layout,
//             entries: &entries,
//         });
//         Ok(Grc::new(bind).into())
//     }
// }

// impl Adapt for BindGroupUnit {
//     fn back(&mut self, back: &Back) {
//         for (_, buffer) in &mut self.entries {
//             buffer.back(back);
//         }
//     }
// }