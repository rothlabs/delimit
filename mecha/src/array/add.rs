use super::*;

/// Add vector to each vector in base.
pub struct Add<N: Number> {
    base: Stem<N>,
    vector: Array1<N>,
}

impl<N: Number> Add<N> {
    pub fn hold(base: &Stem<N>, vector: &Array1<N>) -> Hold<Link<Self>, Role<N>> {
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

impl<N: Number> Grant for Add<N> {
    type Load = Load<N>;
    fn grant(&self) -> Self::Load {
        let mut array = self.base.load().array();
        array.each(|i, b| self.vector.get([i[0]]) + b);
        Ace::new(Bare::Mem(array))
    }
}
