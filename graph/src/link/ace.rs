use crate::*;

pub trait ToAce<L> {
    fn ace(&self) -> Ace<L>;
}

impl<L> ToAce<L> for L
where
    L: Clone,
{
    fn ace(&self) -> Ace<L> {
        Ace::new(self.clone())
    }
}

impl ToAce<String> for str {
    fn ace(&self) -> Ace<String> {
        Ace::new(self.into())
    }
}

pub trait IntoAce<L> {
    fn into_ace(self) -> Ace<L>;
}

impl<L> IntoAce<L> for L {
    fn into_ace(self) -> Ace<L> {
        Ace::new(self)
    }
}