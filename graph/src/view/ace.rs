use crate::*;

/// View of ace view, by ploy.  
pub type Ploy<A, L> = View<role::Ploy<A, link::Ace<L>>, Ace<L>>;

/// View of ace view, by plan.  
pub type Plan<P, T, L> = View<role::Plan<P, T, link::Ace<L>>, Ace<L>>;

/// Ace view. A bare load or `link::Ace<Load>`
/// This is a terminal view that may be at the end of a chain of views.
/// By design, it does not follow the Role-Base structure of the regular View type.
#[derive(Clone, Serialize)]
pub enum Ace<L> {
    Bare(L),
    Link(link::Ace<L>),
}

impl<L> Default for Ace<L>
where
    L: Default,
{
    fn default() -> Self {
        Self::Bare(L::default())
    }
}

impl From<&str> for Ace<String> {
    fn from(value: &str) -> Self {
        Self::Bare(value.into())
    }
}

impl<L> FromAce for Ace<L> {
    type Load = L;
    fn from_ace(ace: link::Ace<L>) -> Self {
        Self::Link(ace)
    }
}

impl<L> Reader for Ace<L>
where
    L: 'static + Send + Sync,
{
    type Item = L;
    fn read<T, F: FnOnce(&L) -> T>(&self, read: F) -> T {
        match self {
            Self::Bare(bare) => read(bare),
            Self::Link(ace) => ace.read(read),
        }
    }
}

impl<L> Backed for Ace<L>
where
    L: Clone,
{
    fn backed(&self, back: &Back) -> Self {
        match self {
            Self::Bare(bare) => Self::Bare(bare.clone()),
            Self::Link(ace) => Self::Link(ace.backed(back)),
        }
    }
}

// impl<L> Grant for Ace<L>
// where
//     L: Clone,
// {
//     type Load = link::Ace<L>;
//     fn grant(&self) -> link::Ace<L> {
//         match self {
//             Self::Bare(bare) => bare.ace(),
//             Self::Link(ace) => ace.clone(),
//         }
//     }
// }

// /// View of ace view, by plan.
// pub type Plan<P, T, L> = View<role::Plan<P, T, link::Ace<L>>, Ace<L>>;

// impl<L> Solve for Ace<L>
// where
//     L: Clone,
// {
//     type Task = ();
//     type Load = link::Ace<L>;
//     fn solve(&self, task: Self::Task) -> Self::Load {
//         match self {
//             Self::Bare(bare) => bare.ace(),
//             Self::Link(ace) => ace.clone(),
//         }
//     }
// }
