use super::*;

pub type Ploy<P, L> = Role<P, link::Ploy<L>>;

/// Two copies of the same link.
/// For Unit access, the part field should be an enumeration of Links.
/// The form field should be link::Ploy or link::Plan to be used without Unit knowledge.
pub struct Role<P, F> {
    pub part: P,
    pub form: F,
}

impl<P, F> Clone for Role<P, F>
where
    P: Clone,
    F: Clone,
{
    fn clone(&self) -> Self {
        Self {
            part: self.part.clone(),
            form: self.form.clone(),
        }
    }
}

impl<P, F> Grant for Role<P, F>
where
    F: Grant,
{
    type Load = F::Load;
    fn grant(&self) -> Self::Load {
        self.form.grant()
    }
}

impl<P, F> Backed for Role<P, F>
where
    P: Clone,
    F: Backed,
{
    fn backed(&self, back: &Back) -> Self {
        Self {
            part: self.part.clone(),
            form: self.form.backed(back),
        }
    }
}

impl<P, L> Serialize for Role<P, L>
where
    P: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.part.serialize(serializer)
    }
}
