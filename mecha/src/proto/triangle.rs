use super::*;

/// Parametric triangle shape.
/// Use the `plot` method to get a point and derivative vectors from UV params.
/// The `intersect` method gives an intersection type with other triangle if any.
#[derive(PartialEq)]
pub struct Triangle {
    a: Vector3,
    b: Vector3,
    c: Vector3,
}

impl Triangle {
    pub fn new(points: &[f64]) -> Self {
        Self {
            a: Vector3::new(&points[..3]),
            b: Vector3::new(&points[3..6]),
            c: Vector3::new(&points[6..]),
        }
    }

    /// Find intersection type with other triangle
    pub fn intersect(&self, rhs: &Self, tol: f64) -> Option<Intersection> {
        let edge = self.intersect_edge(rhs, tol);
        if edge.is_some() {
            edge
        } else {
            self.intersect_cross(rhs, tol)
        }
    }

    /// Find intersection along edge with other triangle
    /// The other triangle must have the same winding, meaning that edge AB goes with BA of other
    fn intersect_edge(&self, rhs: &Self, tol: f64) -> Option<Intersection> {
        // AB
        if ((&self.a - &rhs.a).length() < tol && (&self.b - &rhs.c).length() < tol)
            || ((&self.a - &rhs.b).length() < tol && (&self.b - &rhs.a).length() < tol)
            || ((&self.a - &rhs.c).length() < tol && (&self.b - &rhs.b).length() < tol)
        {
            return Some(Intersection::AB);
        // AC
        } else if ((&self.a - &rhs.a).length() < tol && (&self.c - &rhs.b).length() < tol)
            || ((&self.a - &rhs.b).length() < tol && (&self.c - &rhs.c).length() < tol)
            || ((&self.a - &rhs.c).length() < tol && (&self.c - &rhs.a).length() < tol)
        {
            return Some(Intersection::AC);
        // BC
        } else if ((&self.b - &rhs.a).length() < tol && (&self.c - &rhs.c).length() < tol)
            || ((&self.b - &rhs.b).length() < tol && (&self.c - &rhs.a).length() < tol)
            || ((&self.b - &rhs.c).length() < tol && (&self.c - &rhs.b).length() < tol)
        {
            return Some(Intersection::BC);
        }
        None
    }

    /// Try to find bad crossing intersection
    fn intersect_cross(&self, rhs: &Self, tol: f64) -> Option<Intersection> {
        // setup point A at center of self
        let mut param_a = Param::new(0.5, 0.5);
        let mut point_a = self.plot(&param_a).point;
        // setup point B at center of rhs and hone to A
        let mut param_b = rhs.hone(&Param::new(0.5, 0.5), &point_a);
        let mut point_b = rhs.plot(&param_b).point;
        // Hone points several times because the velocity field of the shape is not constant
        // The triangle could be made with uniform velocity field and it would only take 1 or 2 hone calls.
        for _ in 0..30 {
            // hone A to B
            param_a = self.hone(&param_a, &point_b);
            point_a = self.plot(&param_a).point;
            // hone B to A
            param_b = rhs.hone(&param_b, &point_a);
            point_b = rhs.plot(&param_b).point;
        }
        if (&point_a - &point_b).length() < tol {
            // Only return Intersection::Cross if one of the params are between 0 and 1 exclusive.
            // Two triangles could touch incorrectly at edges but we won't catch that here
            // and allow it to be caught in crossing with other triangle.
            if !param_a.on_edge() || !param_b.on_edge() {
                return Some(Intersection::Cross);
            }
        }
        None
    }

    /// New param adjusted so the plot is closer to target.
    /// If the shape is planar with uniform velocity field, this will result
    /// in a perfect jump to the shape plot that is closest to the target.
    fn hone(&self, param: &Param, target: &Vector3) -> Param {
        let plot = self.plot(param);
        let delta = target - &plot.point;
        let length = delta.length();
        let mut u = param.u;
        let mut v = param.v;
        if let Some(direction) = delta.normalized() {
            if let Some(dir_u) = plot.velocity_u.normalized() {
                let proj_u = dir_u.dot(&direction) * length;
                u += proj_u / plot.velocity_u.length()
            }
            if let Some(dir_v) = plot.velocity_v.normalized() {
                let proj_v = dir_v.dot(&direction) * length;
                v += proj_v / plot.velocity_v.length()
            }
        }
        Param::new(u, v)
    }

    /// New plot from param UV.
    /// This triangle shape does not have a uniform velocity field which makes intersection
    /// testing less efficient. However, it does not require a UV bounding shape and
    /// we know if the plot is on the edge if U or V is 0 or 1.
    fn plot(&self, param: &Param) -> Plot {
        Plot {
            // This interpolates from A to B by U. Then it interpolate from that to C by V.
            point: self.a.lerp(&self.b, param.u).lerp(&self.c, param.v),
            // The U velocity is inversely proportional to V
            velocity_u: &(&self.b - &self.a) * (1. - param.v),
            // The V velocity is interpolated between AC and BC by U
            velocity_v: (&self.c - &self.a).lerp(&(&self.c - &self.b), param.u),
        }
    }
}

/// Triangle intersection types
#[derive(PartialEq, Eq, Hash)]
pub enum Intersection {
    AB,
    AC,
    BC,
    Cross,
}
