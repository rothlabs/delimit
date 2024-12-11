use super::*;

#[derive(Debug, Clone)]
pub struct Entry {
    pub slot: u32,
    pub buffer: Hub<Grc<Buffer>>,
    pub size: Option<NonZero<u64>>,
}

#[derive(Debug, Gate, Back)]
pub struct Bind {
    pub slot: Hub<u32>,
    pub group: Hub<Grc<BindGroup>>,
    pub offsets: Vec<Hub<u32>>,
}

impl Solve for Bind {
    type Base = stable::GroupBind;
    async fn solve(&self) -> node::Result<stable::GroupBind> {
        let mut offsets = self.offsets.base().await?;
        for offset in &mut offsets {
            *offset *= 4;
        }
        let binding = stable::GroupBind {
            slot: self.slot.base().await?,
            group: self.group.base().await?,
            offsets,
        };
        Ok(binding.into())
    }
}

// #[derive(Debug, Gate, Back, TypedBuilder)]
// pub struct Bind {
//     #[builder(default, setter(into))]
//     pub slot: Hub<u32>,
//     #[builder(setter(into))]
//     pub group: Hub<Grc<BindGroup>>,
//     #[builder(default, setter(into))]
//     pub offsets: Vec<Hub<u32>>,
// }

