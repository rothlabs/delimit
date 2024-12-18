use super::*;

mod cusp;

pub type Result<T> = std::result::Result<Hub<T>, Box<dyn std::error::Error + Send + Sync>>;

pub trait Solve {
    type Base;
    fn solve(&self) -> impl Future<Output = agent::Result<Self::Base>>;
}

pub struct Link<T: Solve> {
    edge: Grc<Edge<T>>,
    // root: Root,
}

pub struct Edge<T: Solve> {
    cusp: Grc<Cusp<T>>,
    // back: Option<Back>,
}

pub struct Cusp<T: Solve> {
    unit: T,
    base: Cell<Option<Hub<T::Base>>>,
    ring: u8,
}