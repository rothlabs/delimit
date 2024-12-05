use super::*;

#[derive(Builder, BuildGate, Back, Debug)]
#[builder(pattern = "owned")]
#[builder(setter(into))]
pub struct Grid {
    #[back(skip)]
    mech: Mech,
    #[builder(setter(each(name = "count", into)))]
    counts: Vec<Hub<u32>>,
    shape: Hub<Shape>,
}

impl Solve for Grid {
    type Base = Chart;
    async fn solve(&self) -> node::Result<Self::Base> {
        let shape = self.shape.base().await?;
        let hedge = shape.chart(&self.mech).grid(&self.counts)?;
        let plot = Chart {
            hedge,
            shape: self.shape.clone(),
        };
        Ok(plot.into())
    }
}
