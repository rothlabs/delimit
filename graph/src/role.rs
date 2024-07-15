use super::*;

pub type Ploy<A, L> = Role<A, link::Ploy<L>>;

/// form
/// part
pub struct Role<A, M> {
    pub actual: A,
    pub method: M,
}

impl<A, M> Clone for Role<A, M>
where
    A: Clone,
    M: Clone,
{
    fn clone(&self) -> Self {
        Self {
            actual: self.actual.clone(),
            method: self.method.clone(),
        }
    }
}

impl<A, M> Grant for Role<A, M>
where
    M: Grant,
{
    type Load = M::Load;
    fn grant(&self) -> Self::Load {
        self.method.grant()
    }
}

impl<A, M> Backed for Role<A, M>
where
    A: Clone,
    M: Backed,
{
    fn backed(&self, back: &Back) -> Self {
        Self {
            actual: self.actual.clone(),
            method: self.method.backed(back),
        }
    }
}

impl<A, L> Serialize for Role<A, L>
where
    A: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.actual.serialize(serializer)
    }
}
