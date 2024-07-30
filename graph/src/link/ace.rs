use crate::*;

/// Clone to Ace link.
pub trait ToAce {
    fn ace(&self) -> Ace<Load>;
}

// impl<L> ToAce<L> for L
// where
//     L: Clone,
// {
//     fn ace(&self) -> Ace<L> {
//         Ace::new(self.clone())
//     }
// }

impl ToAce for str {
    fn ace(&self) -> Ace<Load> {
        Ace::new(Load::String(self.into()))
    }
}

// impl ToAce<String> for str {
//     fn ace(&self) -> Ace<String> {
//         Ace::new(self.into())
//     }
// }

/// Move into Ace link.
pub trait IntoAce<L> {
    fn into_ace(self) -> Ace<L>;
}

impl<L> IntoAce<L> for L {
    fn into_ace(self) -> Ace<L> {
        Ace::new(self)
    }
}
