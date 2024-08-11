use super::*;

pub struct Triangle {
    a: Vector3,
    b: Vector3,
    c: Vector3,
}

impl Triangle {
    pub fn new(points: &[f64]) -> Self {
        Self { 
            a: Vector3::new(&points[..2]),
            b: Vector3::new(&points[3..6]),
            c: Vector3::new(&points[6..])
        }
    }
    pub fn plot(&self, u: f64, v: f64) -> Plot {
        let point = self.a.interpolate(&self.b, u).interpolate(&self.c, v);
        let velocity_u = &(&self.b - &self.a) * (1. - v);
        let velocity_v = (&self.c - &self.a).interpolate(&(&self.c - &self.b), u);
        Plot {point, velocity_u, velocity_v}
    }
}