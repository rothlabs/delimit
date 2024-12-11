use super::*;

pub mod command;

#[derive(Debug, Clone)]
pub struct GroupBind {
    pub slot: u32,
    pub group: Grc<BindGroup>,
    pub offsets: Vec<u32>,
}

impl PartialEq for GroupBind {
    fn eq(&self, rhs: &GroupBind) -> bool {
        self.slot == rhs.slot
            && self.group.global_id() == rhs.group.global_id()
            && self.offsets == rhs.offsets
    }
}

#[derive(Clone, Debug)]
pub struct BufferBind {
    pub slot: u32,
    pub buffer: Grc<Buffer>,
    pub offset: Option<u32>,
}

impl PartialEq for BufferBind {
    fn eq(&self, rhs: &BufferBind) -> bool {
        self.slot == rhs.slot && Grc::ptr_eq(&self.buffer, &rhs.buffer)
    }
}
