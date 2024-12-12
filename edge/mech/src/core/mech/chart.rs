use super::*;

#[derive(Gate, Back, Debug)]
pub struct Grid {
    #[back(skip)]
    pub mech: Mech,
    pub shape: Hub<flat::Shape>,
    pub counts: Vec<Hub<u32>>,
}

impl Solve for Grid {
    type Base = crate::Chart;
    async fn solve(&self) -> node::Result<Self::Base> {
        let shape = self.shape.base().await?;
        let hedge = shape.chart(&self.mech).grid(&self.counts)?;
        let plot = crate::Chart {
            hedge,
            shape: self.shape.clone(),
        };
        Ok(plot.into())
    }
}

// #[derive(Builder, BuildGate, Back, Debug)]
// #[builder(pattern = "owned")]
// #[builder(setter(into))]
// pub struct Grid {
//     #[back(skip)]
//     mech: Mech,
//     #[builder(setter(each(name = "count", into)))]
//     counts: Vec<Hub<u32>>,
//     shape: Hub<Shape>,
// }
