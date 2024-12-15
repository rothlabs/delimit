use super::*;

// pub enum Form {
//     Differ()
// }

pub struct Differ<S, T, const K: usize> {
    pub form: differ::Form<T, K>,
    pub stem: S,
}

pub mod differ {
    use super::*;
    pub enum Form<T, const K: usize> {
        Extrude(Grc<Vector<T, K>>),
        Revolve(Revolve<T, K>),
    }
    pub struct Revolve<T, const K: usize> {
        pub axes: [Grc<Vector<T, K>>; K],
        // only used for 2D. Angels encoded in axis magnitude for 3D and up
        pub angle: T,
    }
}

/// Spline of O + 3 order
/// TODO: add another constant for multiple spans (pattern): pub stems_c: [S; M]
#[derive(Debug)]
pub struct Spline<S, const O: usize, T> {
    pub form: spline::Form<T, O>,
    pub stems_a: [S; 3],
    pub stems_b: [S; O],
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
