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
        let chart = &self.view.mech.bank.draw.chart;
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

        let vertex_count: u32 = 8;
        let points = Circle {
            frame: self.view.port.size.clone(), //gpu.display.config.clone(),
            count: vertex_count.into(),
            radius: 4.0.into(),
        }
        .gate()?;
        let buffer = gpu.buffer(vertex_count as u64 * 24).vertex()?;
        let mesh = Hedge {
            root: gpu.writer(buffer.clone()).data(points).hub()?,
            buffer: buffer.into(),
        };

        self.view
            .port
            .command()
            .root(rig.root)
            .root(hedge.root)
            .root(mesh.root.clone())
            // .texture_view(texture_view)
            // .resolve_target(gpu.display.view())
            .render(chart.points.pipe.clone())
            .bind(0, bind)
            .vertex(0, mesh.buffer.clone())
            .draw(0..vertex_count * 3, 0..count)
            .hub()
        // Ok(Mutation.into())
    }
}

use std::f32::consts::PI;

#[derive(Back, Debug)]
pub struct Circle {
    pub frame: Leaf<(u32, u32)>,
    pub count: Hub<u32>,
    pub radius: Hub<f32>,
}

impl GateTag for Circle {}

impl Solve for Circle {
    type Base = Vec<f32>;
    async fn solve(&self) -> graph::Result<Hub<Vec<f32>>> {
        // let (w, h) = self
        //     .display
        //     .read(|display| (display.width as f32, display.height as f32))?;
        let frame = self.frame.base()?;
        let (w, h) = (frame.0 as f32, frame.1 as f32);
        let count = self.count.base().await?;
        let radius = self.radius.base().await?;
        let points = circle_points(count, radius);
        let p0 = points.last().unwrap_or(&(0., 0.));
        let p1 = points.first().unwrap_or(&(0., 0.));
        let mut out = vec![0., 0., p0.0 / w, p0.1 / h, p1.0 / w, p1.1 / h];
        for i in 1..count as usize {
            let p0 = points[i - 1];
            let p1 = points[i];
            out.extend([0., 0., p0.0 / w, p0.1 / h, p1.0 / w, p1.1 / h]);
        }
        Ok(out.into())
    }
}

fn circle_points(count: u32, radius: f32) -> Vec<(f32, f32)> {
    (0..count)
        .map(|i| {
            let angle = (i as f32 / count as f32) * 2.0 * PI;
            let x = radius * angle.cos();
            let y = radius * angle.sin();
            (x, y)
        })
        .collect()
}
