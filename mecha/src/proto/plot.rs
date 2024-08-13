use super::*;

/// Parametric surface plot
/// UV vectors are velocity or derivative by respective param UV.
pub struct Plot {
    pub point: Vector3,
    pub velocity_u: Vector3,
    pub velocity_v: Vector3,
}

/// UV param of parametric surface. Used to produce a Plot.
pub struct Param {
    pub u: f64,
    pub v: f64,
}

impl Param {
    /// Make new param with UV clamped 0 to 1.
    pub fn new(u: f64, v: f64) -> Self {
        Self {
            u: u.min(1.).max(0.),
            v: v.min(1.).max(0.),
        }
    }
    /// True if U or V is close to 0 or 1.
    pub fn on_edge(&self) -> bool {
        let tol = 0.001;
        self.u < tol || self.u > 1. - tol || self.v < tol || self.v > 1. - tol
    }
}
