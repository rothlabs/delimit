pub use layout::*;

use super::*;
use typed_builder::TypedBuilder;

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

#[derive(Debug, Gate, Back)]
pub struct BindGroupUnit {
    #[back(skip)]
    pub device: Grc<Device>,
    #[back(skip)]
    pub layout: BindGroupLayout,
    pub buffers: Vec<Hub<Grc<Buffer>>>,
}

impl Solve for BindGroupUnit {
    type Base = Grc<BindGroup>;
    async fn solve(&self) -> node::Result<Self::Base> {
        // let mut buffers = vec![];
        // for buffer in &self.entries {
        //     buffers.push(buffer.base().await?);
        // }
        let buffers = self.buffers.base().await?;
        let mut entries = vec![];
        for (i, buffer) in buffers.iter().enumerate() {
            let resource = buffer.as_entire_binding();
            entries.push(BindGroupEntry {
                binding: i as u32,
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

// impl Adapt for BindGroupUnit {
//     fn back(&mut self, back: &Back) {
//         for buffer in &mut self.buffers {
//             buffer.back(back);
//         }
//     }
// }

// #[derive(Debug, Gate)]
// pub struct BindGroupUnit {
//     pub device: Grc<Device>,
//     pub layout: BindGroupLayout,
//     pub entries: Vec<(u32, Hub<Grc<Buffer>>)>,
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




// #[derive(TypedBuilder)]
// pub struct BindGroupUnit2 {
//     device: Grc<Device>,
//     layout: BindGroupLayout,
//     entries: Vec<(u32, Hub<Buffer>)>,
// }

// impl Solve for BindGroupUnit2 {
//     type Base = Grc<BindGroup>;
//     async fn solve(&self) -> node::Result<Self::Base> {
//         // let mut buffers = vec![];
//         // for (binding, buffer) in &self.entries {
//         //     buffers.push((*binding, buffer.base().await?));
//         // }
//         // self.entries.read
//         let mut entries = vec![];
//         for (i, buffer) in &buffers {
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
