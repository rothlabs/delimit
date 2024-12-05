use super::*;
use std::f32::consts::PI;

#[derive(Builder, BuildGate, Back, Debug)]
#[builder(pattern = "owned")]
#[builder(setter(into))]
pub struct Points {
    // TODO: switch to Canvas enum of Viewport or TargetTexture
    #[back(skip)]
    view: View,
    plot: Hub<Plot>,
}

impl Solve for Points {
    type Base = Grc<gpu::Action>;
    async fn solve(&self) -> node::Result<Grc<gpu::Action>> {
        let plot = self.plot.base().await?;
        let plot_size = plot.shape.base().await?.plot_size();
        let chart = plot.hedge;
        
        let res: u32 = 8;
        let gpu = &self.view.port.gpu;
        let store = &gpu.store;
        let rig = store.rig_bind(1);
        let mesh_index = store.topic(res * 6);
        let plot_count = chart.size.math().div(plot_size).hub();

        let points = Circle {
            frame: self.view.port.size.clone(),
            count: res.into(),
            radius: 4.0.into(),
        }
        .hub();

        let mesh_stem = gpu
            .writer(&store.topic.buffer)
            .index(&mesh_index)
            .data(points)
            .hub()?;

        let vector = VectorBuilder::default()
            .field(mesh_index)
            .field(chart.index)
            .field(plot_size)
            .field(&plot_count)
            .hub()?;

        let rig_stem = gpu
            .writer(&store.rig.buffer)
            .data(vector)
            .index(rig.index)
            .hub()?;

        let stems = chart.stems.with(&[rig_stem, mesh_stem]);
        // TODO: image could return Image that includes rig offset
        //  this way, rig bind does not need to be provided. perhapes bind group number
        //  would need to be provided with the pipeline
        let pipe = &self.view.mech.bank.draw.chart.points;
        Ok(gpu.image(pipe).basic(gpu::image::Basic {
            stems,
            rig: rig.bind,
            vertices: (res * 3).into(),
            instances: plot_count,
        }))
    }
}

#[derive(Debug, Gate, Back)]
pub struct Circle {
    pub frame: Leaf<(u32, u32)>,
    pub count: Hub<u32>,
    pub radius: Hub<f32>,
}

impl Solve for Circle {
    type Base = Vec<f32>;
    async fn solve(&self) -> node::Result<Vec<f32>> {
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

// let vertex_offset = store.mesh(res * 6); // 24
//         let model_stem = gpu
//             .writer(model_buffer)
//             .offset(&vertex_offset)
//             .data(points)
//             .hub()?;

// Ok(gpu::draw()
//     .stems(stems)
//     .pipe(&chart.points)
//     .bind(&store.topic.bind_vertex)
//     .bind(rig.bind)
//     .buffer(&store.mesh.vertex)
//     .vertex_offset(mesh_offset)
//     .vertex_length(res * 3)
//     .instance_offset(hedge.offset)
//     .instance_length(count)
//     .hub()?)

//         let buffer = gpu.buffer(vertex_count as u64 * 24).vertex()?;
//         let stem = gpu.writer(&buffer).data(points).hub()?;
//         let mesh = gpu::bufferhedge().buffer(buffer).stem(stem).build()?;
//         let stems = hedge.stems.with(&mesh.stems).with(&[uniform_stem]);
//         let vertex = gpu::vertex().buffer(&mesh.buffer).hub()?;

// let texture_view = gpu.display.texture()?.sample_count(4).view()?;

// let mesh = Hedge {
//     stem: gpu.writer(buffer.clone()).data(points).hub()?,
//     buffer: buffer.into(),
// };

// let codec = self
//             .view
//             .port
//             .codec()
//             .stems(stems)
//             // .stem(rig.stem)
//             // .stem(hedge.stem)
//             // .stem(mesh.stem.clone())
//             // .texture_view(texture_view)
//             // .resolve_target(gpu.display.view())
//             .pipe(chart.points.pipe.clone())
//             .bind(0, bind)
//             .vertex(0, mesh.buffer.clone())
//             .draw(0..vertex_count * 3, 0..count)
//             .hub()?;
