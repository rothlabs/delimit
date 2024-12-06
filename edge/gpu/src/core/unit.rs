use super::*;

pub struct BindGroupUnit {
    device: Grc<Device>,
    layout: Grc<BindGroupLayout>,
    entries: Vec<(u32, Hub<Grc<Buffer>>)>,
}

impl Solve for BindGroupUnit {
    type Base = Grc<BindGroup>;
    async fn solve(&self) -> node::Result<Self::Base> {
        let mut buffers = vec![];
        for (binding, buffer) in &self.entries {
            buffers.push((*binding, buffer.base().await?));
        }
        let mut entries = vec![];
        for (i, buffer) in &buffers {
            //let wow = buffer.as_entire_buffer_binding();
            let resource = buffer.as_entire_binding();
            entries.push(BindGroupEntry {
                binding: *i,
                resource,
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

impl Adapt for BindGroupUnit {
    fn back(&mut self, back: &Back) {
        for (_, buffer) in &mut self.entries {
            buffer.back(back);
        }
    }
}