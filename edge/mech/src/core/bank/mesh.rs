use super::*;
use std::f32::consts::PI;

#[derive(Back, Debug)]
pub struct Circle {
    pub count: Hub<u32>,
    pub radius: Hub<f32>,
    pub display: Leaf<SurfaceConfiguration>,
}

impl GateTag for Circle {}

impl Solve for Circle {
    type Base = Vec<f32>;
    async fn solve(&self) -> graph::Result<Hub<Vec<f32>>> {
        let (w, h) = self
            .display
            .read(|display| (display.width as f32, display.height as f32))?;
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
