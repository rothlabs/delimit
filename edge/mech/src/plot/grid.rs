use super::*;

#[derive(Builder, Gate, Back, Debug)]
#[builder(setter(into))]
pub struct Grid {
    count: Hub<u64>,
    shape: Hub<Shape>,
}

impl Solve for Grid {
    type Base = Hedge;
    async fn solve(&self) -> graph::Result<Hub<Hedge>> {
        let count = self.count.base().await?;
        let shape = self.shape.base().await?;
        shape.grid(count).await
    }
}

// impl Adapt for Grid {
//     fn back(&mut self, back: &Back) -> graph::Result<()> {
//         self.shape.back(back)?;
//         self.count.back(back)
//     }
// }
