use super::*;

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
