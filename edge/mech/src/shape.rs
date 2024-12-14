pub trait Rank<const R: usize> {}

pub trait Dimension<const D: usize> {}

pub trait Flatten {
    fn flat(&self) -> Vec<f32>;
}

// pub trait Rank0 {}
// pub trait Rank1 {}
// pub trait Rank2 {}

// pub trait Dim0 {}
// pub trait Dim1 {}
// pub trait Dim2 {}
// pub trait Dim3 {}
