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
    pub fn plot(&self, u: f64, v: f64) {
        //let point = (&self.a + &self.b) * ;
    }
}