use super::*;

pub struct Plot {
    pub core: Core,
    pub shape: Hub<Shape>,
}

impl Plot {
    pub fn grid(self, count: impl Into<Hub<u32>>) -> graph::Result<Hub<crate::Plot>> {
        GridBuilder::default()
            .core(self.core)
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
    core: Core,
    #[back(skip)]
    #[builder(setter(each(name = "count", into)))]
    counts: Vec<Hub<u32>>,
    shape: Hub<Shape>,
}

impl Solve for Grid {
    type Base = crate::Plot;
    async fn solve(&self) -> graph::Result<Hub<Self::Base>> {
        let shape = self.shape.base().await?;
        let hedge = shape.plot(&self.core).grid(&self.counts)?;
        let plot = crate::Plot {
            hedge,
            shape: self.shape.clone(),
        };
        Ok(plot.into())
    }
}

pub struct Draw {
    pub core: Core,
    pub plot: Hub<crate::Plot>,
}

impl Draw {
    pub fn points(self) -> plot::draw::PointsBuilder {
        // , size: impl Into<Hub<f32>>
        plot::draw::PointsBuilder::default()
            .core(self.core)
            .plot(self.plot)
        // .size(size)
    }
}

// Ok(Grid {
//     mech: self.mech,
//     shape: self.shape,
//     count: count.into(),
// }.gate()?.into())
