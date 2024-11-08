use super::*;

#[derive(Builder, Gate, Back, Debug)]
#[builder(pattern = "owned")]
#[builder(setter(into))]
pub struct Grid {
    #[back(skip)]
    core: Core,
    #[builder(setter(each(name = "count", into)))]
    counts: Vec<Hub<u32>>,
    shape: Hub<Shape>,
}

impl Solve for Grid {
    type Base = Plot;
    async fn solve(&self) -> graph::Result<Hub<Self::Base>> {
        let shape = self.shape.base().await?;
        let hedge = shape.chart(&self.core).grid(&self.counts)?;
        let plot = Plot {
            hedge,
            shape: self.shape.clone(),
        };
        Ok(plot.into())
    }
}
