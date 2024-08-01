use crate::*;

/// Clone to Ace link.
pub trait ToAce {
    fn ace(&self) -> Leaf;
}

impl ToAce for str {
    fn ace(&self) -> Leaf {
        Leaf::new(Load::String(self.into()))
    }
}

// impl ToAce<String> for str {
//     fn ace(&self) -> Ace<String> {
//         Ace::new(self.into())
//     }
// }

// / Move into Ace link.
// pub trait IntoAce<L> {
//     fn into_ace(self) -> Ace;
// }

// impl IntoAce for str {
//     fn into_ace(self) -> Ace {
//         Ace::new(Load::String(self.into()))
//     }
// }
