use super::*;
use std::ops;

/// Add vector to each vector in base.
pub struct Add<N> {
    base: Stem<N>,
    vector: Array1<N>,
}

impl<N> Add<N>
where
    N: Copy + Default + ops::Add<N, Output = N> + Send + Sync + 'static,
{
    pub fn new(base: &Stem<N>, vector: &Array1<N>) -> Hold<Link<Self, N>, Role<N>> {
        let link = Link::make(|back| Self {
            base: base.backed(back),
            vector: vector.clone(),
        });
        let role = Role {
            part: Part::Add(link.clone()),
            form: link.ploy(),
        };
        Hold { link, role }
    }
}

impl<N> Grant for Add<N>
where
    N: Copy + Default + ops::Add<N, Output = N> + Send + Sync + 'static,
{
    type Load = Load<N>;
    fn grant(&self) -> Self::Load {
        let mut array = self.base.load().array();
        array.each(|i, b| self.vector.get([i[0]]) + b);
        Bare::Array(array).ace()
    }
}
