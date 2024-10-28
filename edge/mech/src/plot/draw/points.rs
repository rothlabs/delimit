use super::*;

#[derive(Builder, Gate, Back, Debug)]
#[builder(pattern = "owned")]
#[builder(setter(into))]
pub struct Points {
    #[back(skip)]
    core: Core,
    plot: Hub<Plot>,
    size: Hub<f32>,
}

impl Solve for Points {
    type Base = Mutation;
    async fn solve(&self) -> graph::Result<Hub<Mutation>> {
        let view = self.core.gpu.surface.view();
        let plot = self.plot.base().await?;
        let hedge = plot.hedge;
        let shape = plot.shape.base().await?;
        let stride = shape.plot_stride();
        let count = (hedge.buffer.base().await?.size() / stride as u64 / 4) as u32;
        let rig = self.core.gpu.uniform().field(stride).field(count).make()?;
        let vertex = self
            .core
            .gpu
            .vertex_buffer(&[-0.02_f32, -0.02, 0., 0.02, 0.02, -0.02]);
        let bind = self
            .core
            .gpu
            .bind()
            .layout(self.core.bin.plot.draw.points.layout.clone())
            .entry(0, rig.buffer)
            .entry(1, hedge.buffer.clone())
            .hub()?;
        let root = self
            .core
            .gpu
            .command()
            .root(rig.root)
            .root(hedge.root)
            .texture_view(view)
            .render(self.core.bin.plot.draw.points.pipe.clone())
            .bind(0, bind)
            .vertex(0, vertex)
            .draw(0..3, 0..count)
            .hub()?;
        Ok(root)
    }
}
