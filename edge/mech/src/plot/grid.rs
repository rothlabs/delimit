use super::*;

// TODO: find way to remove Builder because it is not needed here
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
        let hedge = shape.plot(&self.mech).grid(count)?;
        Ok(hedge.into())
    }
}
