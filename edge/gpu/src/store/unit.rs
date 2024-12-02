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
            Kind::Uniform(store) => store.grant(size)?.hub(),
        })
    }
}

#[derive(Debug)]
pub enum Kind {
    Uniform(Grc<UniformStore>),
}
