use super::*;

pub mod layout;
pub mod bind;

#[derive(Debug, Clone)]
pub struct Bank {
    pub rig: Hub<Grc<BindGroup>>,
    pub topic: Hub<Grc<BindGroup>>,
    pub image: Hub<Grc<BindGroup>>,
    pub bind: group::bind::Bank,
}

impl Bank {
    pub fn new(bank: &layout::Bank) -> Self {
        let rig = gpu::active::Group {
            device: bank.device.clone(),
            layout: bank.rig.clone(),
            entries: vec![rig_entry(bank.store)],
        }.hub();
        let topic = gpu::active::Group {
            device: bank.device.clone(),
            layout: bank.topic.clone(),
            entries: vec![topic_entry(bank.store)],
        }.hub();
        let image = gpu::active::Group {
            device: bank.device.clone(),
            layout: bank.image.clone(),
            entries: vec![topic_entry(bank.store)],
        }.hub();
        Self {
            bind: group::bind::Bank::new(&topic),
            rig,
            topic,
            image,
        }
    }
}

fn rig_entry(store: &Store) -> gpu::active::group::Entry {
    gpu::active::group::Entry {
        slot: 0,
        buffer: store.rig.buffer.clone().into(),
        size: NonZero::new(256),
    }
}

fn topic_entry(store: &Store) -> gpu::active::group::Entry {
    gpu::active::group::Entry {
        slot: 0,
        buffer: store.topic.buffer.clone().into(),
        size: None,
    }
}
