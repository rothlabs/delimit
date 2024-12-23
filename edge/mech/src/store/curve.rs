use super::*;

pub struct Differ<T, const K: usize> {
    pub origin: Vector<T, K>,
    pub form: differ::Form<T, K>,
}

pub mod differ {
    use super::*;
    pub enum Form<T, const K: usize> {
        Extrude(Vector<T, K>),
        Revolve(Revolve<T, K>),
    }
    pub struct Revolve<T, const K: usize> {
        pub axes: [Vector<T, K>; K],
        // for first angle. Angles encoded in axis magnitude for 4D and up
        pub angle: T,
    }
}

/// Spline of O + 2 order
/// Currently only represents NURBS but could include B-Spline, Catmull–Rom, and others in the future. 
pub struct Spline<T, const K: usize> {
    pub base: spline::Base<T, K>,
    pub parts: Vec<spline::Part<T, K>>,
    pub spans: Vec<spline::Span<T, K>>,
}

pub mod spline {
    use super::*;
    pub struct Control<T, const K: usize> {
        pub vector: Vector<T, K>,
        pub weight: T,
    }

    /// Minimum of 2 controls and 4 knots.
    pub struct Base<T, const K: usize> {
        pub controls: [Control<T, K>; 2],
        pub knots: [T; 4],
    }

    /// Represents an order increase, requiring the addition of one control and two knots.
    pub struct Part<T, const K: usize> {
        pub control: Control<T, K>,
        pub knots: [T; 2],
    }

    /// Represents a segment of the curve by addition of a control and knot.
    pub struct Span<T, const K: usize> {
        pub control: Control<T, K>,
        pub knot: T,
    }
}

// pub enum Control<T, K> {
//     Direct(Vector<T, K>),
//     Rational(Rational<T, K>)
// }