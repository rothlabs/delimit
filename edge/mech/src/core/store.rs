use super::*;

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
