use super::*;

#[derive(Debug, Gate, Back)]
pub struct Grant {
    pub size: Hub<u32>,
    #[back(skip)]
    pub kind: Kind,
}

impl Solve for Grant {
    type Base = u32;
    async fn solve(&self) -> node::Result<u32> {
        let size = self.size.base().await?;
        Ok(match &self.kind {
            Kind::Rig(store) => store.grant()?.hub(),
            Kind::Topic(store) => store.grant(size)?.hub(),
            Kind::Mesh(store) => store.grant(size)?.hub(),
        })
    }
}
