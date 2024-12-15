use super::*;

pub struct Differ<T, const D: usize, S> {
    pub stem: S,
    pub kind: differ::Kind<T, D>,
}

mod differ {
    use super::*;
    pub enum Kind<T, const D: usize> {
        Extrude(Grc<Vector<T, D>>),
        Revolve(Grc<Matrix<T, D>>),
    }
}

pub struct Spline<T, const D: usize, S, const O: usize> {
    pub stems: [S; O],
    pub kind: spline::Kind<T, D>,
}

mod spline {
    use super::*;
    pub enum Kind<T, const D: usize> {
        Wow(Grc<Vector<T, D>>),
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
