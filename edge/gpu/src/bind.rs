pub use layout::*;

use super::*;

mod layout;

#[derive(Builder, BuildGate, Debug)]
#[builder(pattern = "owned")]
#[builder(setter(into, strip_option))]
pub struct BindGroupRig {
    device: Grc<Device>,
    #[builder(default)]
    layout: Option<Grc<BindGroupLayout>>,
    #[builder(default)]
    pipe: Option<Grc<ComputePipeline>>,
    #[builder(default, setter(each(name = "inner_entry", into)))]
    entries: Vec<(u32, Hub<Grc<Buffer>>)>,
}

impl Solve for BindGroupRig {
    // TODO: make a gpu::BindGroup that includes mutations from Hedge entries instead of Buffer entries
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
        let layout = if let Some(layout) = &self.layout {
            layout.as_ref()
        } else if let Some(pipe) = &self.pipe {
            &pipe.get_bind_group_layout(0)
        } else {
            Err(anyhow!("No BindGroupLayout in Bind"))?
        };
        let bind = self.device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout,
            entries: &entries,
        });
        Ok(Grc::new(bind).into())
    }
}

impl Adapt for BindGroupRig {
    fn back(&mut self, back: &Back) {
        for (_, buffer) in &mut self.entries {
            buffer.back(back);
        }
    }
}

impl BindGroupRigBuilder {
    pub fn entry(self, i: u32, buffer: impl Into<Hub<Grc<Buffer>>>) -> Self {
        self.inner_entry((i, buffer.into()))
    }
}
