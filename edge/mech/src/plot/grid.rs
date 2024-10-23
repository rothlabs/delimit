use super::*;

#[derive(Builder, Gate, Back, Debug)]
#[builder(pattern = "owned")]
#[builder(setter(into))]
pub struct Grid {
    count: Hub<u32>,
    shape: Hub<Shape>,
}

impl Solve for Grid {
    type Base = Hedge;
    async fn solve(&self) -> graph::Result<Hub<Hedge>> {
        let shape = self.shape.base().await?;
        Ok(shape.grid(self.count.clone())?.into())
    }
}