use super::*;

/// Parametric surface plot
/// UV vectors are velocity or derivative by respective param UV.
pub struct Plot {
    pub point: Vector3,
    pub vector_u: Vector3,
    pub vector_v: Vector3,
}

/// UV param of parametric surface. Used to produce a Plot.
pub struct Param {
    pub u: f64,
    pub v: f64,
}

impl Param {
    pub fn new(u: f64, v: f64) -> Self {
        Self { u, v }
    }
    pub fn on_edge(&self) -> bool {
        let tol = 0.001;
        self.u < tol || self.u > 1. - tol || self.v < tol || self.v > 1. - tol
    }
}
