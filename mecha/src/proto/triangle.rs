use std::f64::EPSILON;

use super::*;

/// Parametric triangle.
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

    /// Find intersection with other triangle
    pub fn intersect(&self, rhs: &Self, tol: f64) -> Option<(Intersection, Vector3, Vector3)> {
        let edge = self.intersect_edge(rhs, tol);
        if edge.is_some() {
            edge
        } else {
            self.intersect_other(rhs, tol)
        }
    }

    /// Find intersection along edge with other triangle
    fn intersect_edge(&self, rhs: &Self, tol: f64) -> Option<(Intersection, Vector3, Vector3)> {
        // AB
        if ((&self.a - &rhs.a).length() < tol && (&self.b - &rhs.c).length() < tol)
            || ((&self.a - &rhs.b).length() < tol && (&self.b - &rhs.a).length() < tol)
            || ((&self.a - &rhs.c).length() < tol && (&self.b - &rhs.b).length() < tol)
        {
            return Some((Intersection::AB, self.a.clone(), self.b.clone()));
        // AC
        } else if ((&self.a - &rhs.a).length() < tol && (&self.c - &rhs.b).length() < tol)
            || ((&self.a - &rhs.b).length() < tol && (&self.c - &rhs.c).length() < tol)
            || ((&self.a - &rhs.c).length() < tol && (&self.c - &rhs.a).length() < tol)
        {
            return Some((Intersection::AC, self.a.clone(), self.c.clone()));
        // BC
        } else if ((&self.b - &rhs.a).length() < tol && (&self.c - &rhs.c).length() < tol)
            || ((&self.b - &rhs.b).length() < tol && (&self.c - &rhs.a).length() < tol)
            || ((&self.b - &rhs.c).length() < tol && (&self.c - &rhs.b).length() < tol)
        {
            return Some((Intersection::BC, self.b.clone(), self.c.clone()));
        }
        None
    }

    /// Find any intersection besides edge intersections
    fn intersect_other(&self, rhs: &Self, tol: f64) -> Option<(Intersection, Vector3, Vector3)> {
        // setup point A at center of self
        let mut param_a = Param::new(0.5, 0.5);
        let mut point_a = self.plot(&param_a).point;
        // setup point B at center of rhs and hone to A
        let mut param_b = rhs.hone(&Param::new(0.5, 0.5), &point_a);
        let mut point_b = rhs.plot(&param_b).point;
        for _ in 0..20 {
            // hone A to B
            param_a = self.hone(&param_a, &point_b);
            point_a = self.plot(&param_a).point;
            // hone B to A
            param_b = rhs.hone(&param_b, &point_a);
            point_b = rhs.plot(&param_b).point;
        }
        if point_a.is_nan() || point_b.is_nan() {
            panic!("point is NaN!!!");       
        }
        if (&point_a - &point_b).length() < tol {
            
            // Only Intersection::Other if one of the params are between 0 and 1 exclusive
            // Two triangles could touch incorrectly at edges but we won't catch that here
            // and allow it to be caught in full crossing with other triangle
            if !param_a.on_edge() || !param_b.on_edge() {
                return Some((Intersection::Other, point_a, point_b));
            }
        }
        None
    }

    /// New param adjusted so the plot is closer to target
    fn hone(&self, param: &Param, target: &Vector3) -> Param {
        let plot = self.plot(param);
        let delta = target - &plot.point;
        let length = delta.length();
        let mut u = param.u;
        let mut v = param.v;
        if let Some(direction) = delta.normalized() {
            if let Some(dir_u) = plot.vector_u.normalized() {
                let proj_u = dir_u.dot(&direction) * length;
                u += proj_u / plot.vector_u.length()
            }
            if let Some(dir_v) = plot.vector_v.normalized() {
                let proj_v = dir_v.dot(&direction) * length;
                v += proj_v / plot.vector_v.length()
            }
        }
        if u.is_nan() || v.is_nan() {
            panic!("uv is NaN!!!");       
        }
        Param {
            u: u.min(1.).max(0.),
            v: v.min(1.).max(0.),
        }
    }

    /// New plot from param UV
    fn plot(&self, param: &Param) -> Plot {
        Plot {
            point: self.a.lerp(&self.b, param.u).lerp(&self.c, param.v),
            vector_u:  &(&self.b - &self.a) * (1. - param.v),
            vector_v: (&self.c - &self.a).lerp(&(&self.c - &self.b), param.u),
        }
    }
}

/// Triangle intersection types
#[derive(PartialEq, Eq, Hash)]
pub enum Intersection {
    Other,
    AB,
    AC,
    BC,
}

// if (&self.a - &rhs.a).length() < tol && (&self.b - &rhs.c).length() < tol {
//     return Some(Intersection::AB);
// } else if (&self.a - &rhs.b).length() < tol && (&self.b - &rhs.a).length() < tol {
//     return Some(Intersection::AB);
// } else if (&self.a - &rhs.c).length() < tol && (&self.b - &rhs.b).length() < tol {
//     return Some(Intersection::AB);
// // AC
// } else if (&self.a - &rhs.a).length() < tol && (&self.c - &rhs.b).length() < tol {
//     return Some(Intersection::AC);
// } else if (&self.a - &rhs.b).length() < tol && (&self.c - &rhs.c).length() < tol {
//     return Some(Intersection::AC);
// } else if (&self.a - &rhs.c).length() < tol && (&self.c - &rhs.a).length() < tol {
//     return Some(Intersection::AC);
// // BC
// } else if (&self.b - &rhs.a).length() < tol && (&self.c - &rhs.c).length() < tol {
//     return Some(Intersection::BC);
// } else if (&self.b - &rhs.b).length() < tol && (&self.c - &rhs.a).length() < tol {
//     return Some(Intersection::BC);
// } else if (&self.b - &rhs.c).length() < tol && (&self.c - &rhs.b).length() < tol {
//     return Some(Intersection::BC);
// }
