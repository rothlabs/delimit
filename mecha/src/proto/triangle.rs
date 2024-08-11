use super::*;

pub struct Triangle<N> {
    a: Vector3<N>,
    b: Vector3<N>,
    c: Vector3<N>,
}

impl<N> Triangle<N> 
where 
    N: Number
{
    pub fn new(points: &[N]) -> Self {
        Self { 
            a: Vector3::new(&points[..2]),
            b: Vector3::new(&points[3..6]),
            c: Vector3::new(&points[6..])
        }
    }
    pub fn plot(&self, u: f64, v: f64) -> Plot<N> {
        let point = self.a.interpolate(&self.b, u).interpolate(&self.c, v);
        let velocity_u = &(&self.b - &self.a) * (1. - v);
        let velocity_v = (&self.c - &self.a).interpolate(&(&self.c - &self.b), u);
        Plot {point, velocity_u, velocity_v}
    }
}