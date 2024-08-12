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
            a: Vector3::new(&points[..2]),
            b: Vector3::new(&points[3..6]),
            c: Vector3::new(&points[6..]),
        }
    }

    /// Find intersection with other triangle
    pub fn intersect(&self, rhs: &Self, tol: f64) -> Option<Intersection> {
        let edge = self.intersect_edge(rhs, tol);
        if edge.is_some() {
            edge
        } else {
            self.intersect_other(rhs, tol)
        }
    }

    /// Find intersection along edge with other triangle
    fn intersect_edge(&self, rhs: &Self, tol: f64) -> Option<Intersection> {
        // AB
        if (&self.a - &rhs.a).length() < tol && (&self.b - &rhs.c).length() < tol {
            return Some(Intersection::AB);
        } else if (&self.a - &rhs.b).length() < tol && (&self.b - &rhs.a).length() < tol {
            return Some(Intersection::AB);
        } else if (&self.a - &rhs.c).length() < tol && (&self.b - &rhs.b).length() < tol {
            return Some(Intersection::AB);
        // AC
        } else if (&self.a - &rhs.a).length() < tol && (&self.c - &rhs.b).length() < tol {
            return Some(Intersection::AC);
        } else if (&self.a - &rhs.b).length() < tol && (&self.c - &rhs.c).length() < tol {
            return Some(Intersection::AC);
        } else if (&self.a - &rhs.c).length() < tol && (&self.c - &rhs.a).length() < tol {
            return Some(Intersection::AC);
        // BC
        } else if (&self.b - &rhs.a).length() < tol && (&self.c - &rhs.c).length() < tol {
            return Some(Intersection::BC);
        } else if (&self.b - &rhs.b).length() < tol && (&self.c - &rhs.a).length() < tol {
            return Some(Intersection::BC);
        } else if (&self.b - &rhs.c).length() < tol && (&self.c - &rhs.b).length() < tol {
            return Some(Intersection::BC);
        }
        None
    }

    /// Find any intersection besides edge intersections
    fn intersect_other(&self, rhs: &Self, tol: f64) -> Option<Intersection> {
        let mut param_a = Param::new(0.5, 0.5);
        let plot_a = self.plot(&param_a);
        let param_b = rhs.hone(&Param::new(0., 0.), &plot_a.point);
        let point_b = rhs.plot(&param_b).point;
        param_a = self.hone(&param_a, &point_b);
        let point_a = self.plot(&param_a).point;
        if (&point_a - &point_b).length() < tol {
            Some(Intersection::Other)
        } else {
            None
        }
    }

    /// New param adjusted so the plot is closer to target
    fn hone(&self, param: &Param, target: &Vector3) -> Param {
        let plot = self.plot(param);
        let delta = target - &plot.point;
        let length = delta.length();
        let direction = delta.normalized();
        let proj_u = plot.vector_u.normalized().dot(&direction) * length;
        let proj_v = plot.vector_v.normalized().dot(&direction) * length;
        Param {
            u: (param.u + proj_u / plot.vector_u.length()).min(0.).max(1.),
            v: (param.v + proj_v / plot.vector_v.length()).min(0.).max(1.),
        }
    }

    /// New plot from param UV
    fn plot(&self, param: &Param) -> Plot {
        let point = self.a.lerp(&self.b, param.u).lerp(&self.c, param.v);
        let velocity_u = &(&self.b - &self.a) * (1. - param.v);
        let velocity_v = (&self.c - &self.a).lerp(&(&self.c - &self.b), param.u);
        Plot {
            point,
            vector_u: velocity_u,
            vector_v: velocity_v,
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
