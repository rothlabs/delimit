use super::*;

#[derive(Builder, Gate, Back, Debug)]
#[builder(pattern = "owned")]
#[builder(setter(into))]
pub struct Points {
    #[back(skip)]
    core: Core,
    plot: Hub<Plot>,
}

// TODO: update to builder encode::render::Command node
impl Solve for Points {
    type Base = Mutation;
    async fn solve(&self) -> graph::Result<Hub<Mutation>> {
        let gpu = &self.core.gpu;
        let draw = &self.core.bank.draw;
        let plot = self.plot.base().await?;
        let hedge = plot.hedge;
        let stride = plot.shape.base().await?.stride();
        let count = (hedge.buffer.base().await?.size() / stride as u64 / 4) as u32;
        let rig = gpu.uniform().field(stride).field(count).make()?;
        let bind = gpu
            .bind()
            .layout(draw.points.layout.clone())
            .entry(0, rig.buffer)
            .entry(1, hedge.buffer.clone())
            .hub()?;
        let texture_view = gpu.display.texture()?.sample_count(4).view()?;
        gpu.compute()
            .root(rig.root)
            .root(hedge.root)
            .root(draw.points.mesh.root.clone())
            // .texture_view(texture_view)
            // .resolve_target(gpu.display.view())
            // .render(draw.points.pipe.clone())
            // .bind(0, bind)
            // .vertex(0, draw.points.mesh.buffer.clone())
            // .draw(0..draw.points.vertex_count, 0..count)
            .hub()
    }
}
