use super::*;

pub mod layout;

pub struct Bank {
    pub rig: Hub<Grc<BindGroup>>,
    pub topic: Topic,
}

pub struct Topic {
    pub normal: Hub<Grc<BindGroup>>,
    pub reader: Hub<Grc<BindGroup>>,
}

impl Topic {
    pub fn new(layout: &layout::Bank) {
        let normal = gpu::group::GroupUnit {
            device: layout.device.clone(),
            layout: layout.topic.clone(),
            entries: vec![],
        };
    }
}

