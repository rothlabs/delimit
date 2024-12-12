use super::*;

pub mod group;
pub mod pipe;

mod chart;

#[derive(Debug)]
pub struct Pipe {
    pub chart: pipe::Chart,
    pub image: pipe::Image,
}

impl Pipe {
    pub fn new(layout: &group::Layout) -> Self {
        Self {
            chart: pipe::Chart::new(layout),
            image: pipe::Image::new(layout),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Group {
    pub rig: Hub<Grc<BindGroup>>,
    pub topic: Hub<Grc<BindGroup>>,
    pub image: Hub<Grc<BindGroup>>,
    pub bind: group::Bind,
}

impl Group {
    pub fn new(bank: &group::Layout) -> Self {
        let rig = gpu::active::Group {
            device: bank.device.clone(),
            layout: bank.rig.clone(),
            entries: vec![rig_entry(bank.store)],
        }
        .hub();
        let topic = gpu::active::Group {
            device: bank.device.clone(),
            layout: bank.topic.clone(),
            entries: vec![topic_entry(bank.store)],
        }
        .hub();
        let image = gpu::active::Group {
            device: bank.device.clone(),
            layout: bank.image.clone(),
            entries: vec![topic_entry(bank.store)],
        }
        .hub();
        Self {
            bind: group::Bind::new(&topic, &image),
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

#[derive(Debug, Clone)]
pub struct Store {
    pub rig: Grc<Shelf>,
    pub topic: Grc<Shelf>,
}

impl Store {
    pub fn new(gpu: &Gpu) -> Self {
        let rig = Shelf::new(gpu);
        let topic = Shelf::new(gpu);
        Self {
            rig: Grc::new(rig),
            topic: Grc::new(topic),
        }
    }
    pub fn rig(&self, size: impl Into<Hub<u32>>) -> Hub<u32> {
        let size: Hub<u32> = size.into();
        self.rig.grant(size.math().mul(64).hub())
    }
    pub fn topic(&self, size: impl Into<Hub<u32>>) -> Hub<u32> {
        self.topic.grant(size)
    }
}

pub struct Chart<'a> {
    pub mech: &'a Mech,
    pub shape: Hub<flat::Shape>,
}

impl Chart<'_> {
    pub fn grid(self, count: impl Into<Hub<u32>>) -> Hub<crate::Chart> {
        chart::Grid {
            mech: self.mech.clone(),
            shape: self.shape,
            counts: vec![count.into()],
        }
        .hub()
    }
}
