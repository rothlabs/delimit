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
        shape.grid(self.count.clone()).await
    }
}

// impl Adapt for Grid {
//     fn back(&mut self, back: &Back) -> graph::Result<()> {
//         self.shape.back(back)?;
//         self.count.back(back)
//     }
// }
