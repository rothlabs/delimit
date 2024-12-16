use super::*;

// TODO: macro to make higher order splines
#[derive(Debug)]
pub enum Form<T, const K: usize> {
    Point(Grc<Vector<T, K>>),
    Differ(Grc<Differ<T, K>>),
    Spline0(Grc<Spline<T, K, 0>>),
    Spline1(Grc<Spline<T, K, 1>>),
}

impl<T, const K: usize> From<Grc<Vector<T, K>>> for Form<T, K> {
    fn from(value: Grc<Vector<T, K>>) -> Self {
        Self::Point(value)
    }
}

impl<T, const K: usize> From<Grc<Differ<T, K>>> for Form<T, K> {
    fn from(value: Grc<Differ<T, K>>) -> Self {
        Self::Differ(value)
    }
}

impl<T, const K: usize> From<Grc<Spline<T, K, 0>>> for Form<T, K> {
    fn from(value: Grc<Spline<T, K, 0>>) -> Self {
        Self::Spline0(value)
    }
}

#[derive(Debug)]
pub struct Differ<T, const K: usize> {
    pub form: differ::Form<T, K>,
    pub stem: Form<T, K>,
}

pub mod differ {
    use super::*;
    #[derive(Debug)]
    pub enum Form<T, const K: usize> {
        Extrude(Grc<Vector<T, K>>),
        Revolve(Revolve<T, K>),
    }
    #[derive(Debug)]
    pub struct Revolve<T, const K: usize> {
        pub axes: [Grc<Vector<T, K>>; K],
        // only used for 2D. Angels encoded in axis magnitude for 3D and up
        pub angle: T,
    }
}

/// Spline of O + 3 order
/// TODO: add another constant for multiple spans (pattern): pub stems_c: [S; M]
#[derive(Debug)]
pub struct Spline<T, const K: usize, const O: usize> {
    pub form: spline::Form<T, O>,
    pub stems_a: [Form<T, K>; 3],
    pub stems_b: [Form<T, K>; O],
}

pub mod spline {
    use super::*;
    #[derive(Debug)]
    pub enum Form<T, const O: usize> {
        Basis(Grc<Knots<T, O>>),
        Nurbs(Nurbs<T, O>),
    }

    #[derive(Debug)]
    pub struct Nurbs<T, const O: usize> {
        pub knots: Grc<Knots<T, O>>,
        pub weights: Grc<Weights<T, O>>,
    }

    /// Knot Vector of order O + 3
    #[derive(Debug)]
    pub struct Knots<T, const O: usize> {
        pub id: u32,
        pub a: [T; 6],
        pub b: [(T, T); O],
    }

    impl<T, const O: usize> Knots<T, O> {
        pub fn new(a: [T; 6], b: [(T, T); O]) -> Grc<Self> {
            Self {
                id: rand::random(),
                a,
                b,
            }
            .into()
        }
    }

    /// Weight Vector of order O + 3
    #[derive(Debug)]
    pub struct Weights<T, const O: usize> {
        pub id: u32,
        pub a: [T; 3],
        pub b: [T; O],
    }
}

// #[derive(Debug)]
// pub struct Revolve<T, const D: usize> {
//     pub vector: Grc<Vector<T, D>>,
//     pub stem: T,
// }

// pub enum Kind<T, const D: usize> {
//     Axis(Axis<T, D>),
// }

// pub struct Axis<T, const D: usize> {
//     pub vector: Grc<Vector<T, D>>,
//     pub kind: axis::Kind<T>,
// }

// mod axis {
//     pub enum Kind<T> {
//         Extrude,
//         Scale(T),
//     }
// }
