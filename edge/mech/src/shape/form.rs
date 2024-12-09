use super::*;

/// Design used to make the vector of vector addition.
#[derive(Builder, Clone, Default, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "graph::Error"))]
#[builder(setter(strip_option))]
pub struct Travel {
    /// Direction and Length Extrusion Vector
    pub extrude: Option<Hedge>,
    // per component function
    // https://www.youtube.com/watch?v=AjDU7eegt4g
}

/// Design used to make the matrix of matrix-vector multiplication.
#[derive(Builder, Clone, Default, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "graph::Error"))]
#[builder(setter(strip_option))]
pub struct Orient {
    /// Axis-Angle Rotation Matrix
    pub revolve: Option<Hedge>,
}

/// Design used to make the vector of matrix-vector multiplication.
#[derive(Builder, Clone, Default, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "graph::Error"))]
#[builder(setter(strip_option))]
pub struct Spline {
    // pub order: u32,
    /// Basis Spline (B-Spline)
    pub basis: Option<Hedge>,
    /// Non-Uniform Rational Basis Spline
    pub nurbs: Option<Hedge>,
}

impl Spline {
    pub fn nurbs_size(&self, mul: &Hub<u32>) -> Hub<u32> {
        if let Some(nurbs) = &self.nurbs {
            nurbs.size.math().mul(mul).hub()
            // gpu::size().buffer(&nurbs.buffer).mul(mul).hub()?
        } else {
            0.into()
        }
    }
}
