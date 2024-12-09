use super::*;

#[derive(Debug, Gate, Back)]
pub struct Grant {
    pub size: Hub<u32>,
    #[back(skip)]
    pub block: Block,
}

impl Solve for Grant {
    type Base = u32;
    async fn solve(&self) -> node::Result<u32> {
        let size = self.size.base().await?;
        Ok(self.block.grant(size)?.hub())
    }
}
