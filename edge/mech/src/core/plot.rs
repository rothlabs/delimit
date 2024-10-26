use super::*;

pub struct Plot {
    pub mech: Core,
    pub shape: Hub<Shape>,
}

impl Plot {
    pub fn grid(self, count: impl Into<Hub<u32>>) -> graph::Result<Hub<Hedge>> {
        GridBuilder::default()
            .mech(self.mech)
            .shape(self.shape)
            .count(count)
            .hub()
    }
}

#[derive(Builder, Gate, Back, Debug)]
#[builder(pattern = "owned")]
#[builder(setter(into))]
pub struct Grid {
    #[back(skip)]
    pub mech: Core,
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

// Ok(Grid {
//     mech: self.mech,
//     shape: self.shape,
//     count: count.into(),
// }.gate()?.into())
