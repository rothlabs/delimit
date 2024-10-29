use super::*;

#[derive(Builder, Gate, Back, Debug)]
#[builder(pattern = "owned")]
#[builder(setter(into))]
pub struct Points {
    #[back(skip)]
    core: Core,
    plot: Hub<Plot>,
}

impl Solve for Points {
    type Base = Mutation;
    async fn solve(&self) -> graph::Result<Hub<Mutation>> {
        let plot = self.plot.base().await?;
        let hedge = plot.hedge;
        let stride = plot.shape.base().await?.plot_stride();
        let count = (hedge.buffer.base().await?.size() / stride as u64 / 4) as u32;
        let rig = self.core.gpu.uniform().field(stride).field(count).make()?;
        let bind = self
            .core
            .gpu
            .bind()
            .layout(self.core.bank.plot.draw.points.layout.clone())
            .entry(0, rig.buffer)
            .entry(1, hedge.buffer.clone())
            .hub()?;
        let texture_view = self.core.gpu.display.texture()?.sample_count(4).view()?;
        let verts = self.core.bank.plot.draw.points.vertex_count;
        self.core
            .gpu
            .command()
            .root(rig.root)
            .root(hedge.root)
            .root(self.core.bank.plot.draw.points.mesh.root.clone())
            .texture_view(texture_view)
            .resolve_target(self.core.gpu.display.view())
            .render(self.core.bank.plot.draw.points.pipe.clone())
            .bind(0, bind)
            .vertex(0, self.core.bank.plot.draw.points.mesh.buffer.clone())
            .draw(0..verts, 0..count)
            .hub()
    }
}
