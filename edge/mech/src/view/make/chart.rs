use super::*;

#[derive(Builder, Gate, Back, Debug)]
#[builder(pattern = "owned")]
#[builder(setter(into))]
pub struct Points {
    // TODO: switch to Canvas enum of Viewport or TargetTexture
    #[back(skip)]
    view: View,
    plot: Hub<Plot>,
}

impl Solve for Points {
    type Base = Mutation;
    async fn solve(&self) -> graph::Result<Hub<Mutation>> {
        let gpu = &self.view.port.gpu;
        let chart = &self.view.bank.chart;
        let plot = self.plot.base().await?;
        let hedge = plot.hedge;
        let stride = plot.shape.base().await?.stride();
        let count = (hedge.buffer.base().await?.size() / stride as u64 / 4) as u32;
        let rig = gpu.uniform().field(stride).field(count).make()?;
        let bind = gpu
            .bind()
            .layout(chart.points.layout.clone())
            .entry(0, rig.buffer)
            .entry(1, hedge.buffer.clone())
            .hub()?;
        // let texture_view = gpu.display.texture()?.sample_count(4).view()?;
        self.view.port.command()
            .root(rig.root)
            .root(hedge.root)
            .root(chart.points.mesh.root.clone())
            // .texture_view(texture_view)
            // .resolve_target(gpu.display.view())
            .render(chart.points.pipe.clone())
            .bind(0, bind)
            .vertex(0, chart.points.mesh.buffer.clone())
            .draw(0..chart.points.vertex_count, 0..count)
            .hub()
        // Ok(Mutation.into())
    }
}
