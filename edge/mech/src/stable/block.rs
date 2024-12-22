use super::*;

// Matrix-based transformation
#[derive(Debug)]
pub struct Differ<S, T, const K: usize> {
    pub kind: differ::Kind<T, K>,
    pub stem: Grc<S>,
}

pub mod differ {
    use super::*;

    #[derive(Debug)]
    pub enum Kind<T, const K: usize> {
        Extrude(Grc<Vector<T, K>>),
        Revolve(Revolve<T, K>),
    }

    #[derive(Debug)]
    pub struct Revolve<T, const K: usize> {
        pub axes: [Grc<Vector<T, K>>; K],
        // only used for 2D. Angles encoded in axis magnitude for 3D and up
        pub angle: T,
    }
}

/// Spline of O + 3 order
/// TODO: add another constant for multiple spans (pattern): pub stems_c: [Grc<S>; M]
#[derive(Debug)]
pub struct Spline<S, T, const O: usize> {
    pub kind: spline::Kind<T, O>,
    pub stems_a: [Grc<S>; 3],
    pub stems_b: [Grc<S>; O],
}

pub mod spline {
    use super::*;

    #[derive(Debug)]
    pub enum Kind<T, const O: usize> {
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
