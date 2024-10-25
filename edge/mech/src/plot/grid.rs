use super::*;

#[derive(Builder, Gate, Back, Debug)]
#[builder(pattern = "owned")]
#[builder(setter(into))]
pub struct Grid {
    #[back(skip)]
    mech: Mech,
    #[back(skip)]
    count: Hub<u32>,
    shape: Hub<Shape>,
}

impl Solve for Grid {
    type Base = Hedge;
    async fn solve(&self) -> graph::Result<Hub<Hedge>> {
        let shape = self.shape.base().await?;
        let count = self.count.clone();
        let hedge = self.mech.plot(&shape).grid(count)?.into();
        Ok(hedge)
        // Ok(shape.grid(self.count.clone())?.into())
    }
}
