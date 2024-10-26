use super::*;

#[derive(Builder, Gate, Back, Debug)]
#[builder(pattern = "owned")]
#[builder(setter(into))]
pub struct Grid {
    #[back(skip)]
    pub mech: Mech,
    #[back(skip)]
    pub count: Hub<u32>,
    pub shape: Hub<Shape>,
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
