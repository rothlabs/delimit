use super::*;

#[derive(Clone, Debug)]
pub struct Plot {
    pub hedge: Hedge,
    pub shape: Hub<Shape>,
}

// impl Plot {
//     fn stride(&self) -> u32 {

//     }
// }

#[derive(Builder, Gate, Back, Debug)]
#[builder(pattern = "owned")]
#[builder(setter(into))]
pub struct Points {
    #[back(skip)]
    core: Core,
    plot: Hub<Plot>,
    size: Hub<u32>,
}

impl Solve for Points {
    type Base = Drawing;
    async fn solve(&self) -> graph::Result<Hub<Drawing>> {
        let plot = self.plot.base().await?;
        let hedge = plot.hedge;
        let shape = plot.shape.base().await?;
        let rank = shape.rank();
        let dimension = shape.dimension;
        let stride = shape.plot_stride(); 
        let index = self.core.gpu.blank(hedge.buffer).div(stride).hub()?;
        Ok(Drawing{}.into())
    }
}
