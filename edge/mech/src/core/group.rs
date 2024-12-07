use super::*;

pub mod layout;

pub struct Bank {
    pub rig: Hub<Grc<BindGroup>>,
    pub topic: Hub<Grc<BindGroup>>,
    pub image: Hub<Grc<BindGroup>>,
}

impl Bank {
    pub fn new(bank: &layout::Bank) -> Self {
        let rig = gpu::active::Group {
            device: bank.device.clone(),
            layout: bank.rig.clone(),
            entries: vec![rig_entry(bank.store)],
        };
        let topic = gpu::active::Group {
            device: bank.device.clone(),
            layout: bank.topic.clone(),
            entries: vec![topic_entry(bank.store)],
        };
        let image = gpu::active::Group {
            device: bank.device.clone(),
            layout: bank.image.clone(),
            entries: vec![topic_entry(bank.store)],
        };
        Self {
            rig: rig.hub(),
            topic: topic.hub(),
            image: image.hub(),
        }
    }
}

fn rig_entry(store: &Store) -> gpu::active::GroupEntry {
    gpu::active::GroupEntry {
        slot: 0,
        buffer: store.rig.buffer.clone().into(),
        size: NonZero::new(256),
    }
}

fn topic_entry(store: &Store) -> gpu::active::GroupEntry {
    gpu::active::GroupEntry {
        slot: 0,
        buffer: store.topic.buffer.clone().into(),
        size: None,
    }
}
