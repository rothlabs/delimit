use crate::*;

/// A bare load or ace link to load.
/// This can be used at the end of a chain of views.
#[derive(Clone, Serialize)]
pub enum End<L> {
    Bare(L),
    Link(Ace<L>),
}

impl<L> Default for End<L>
where
    L: Default,
{
    fn default() -> Self {
        Self::Bare(L::default())
    }
}

impl<T> From<T> for End<T> {
    fn from(value: T) -> Self {
        Self::Bare(value)
    }
}

impl<L> FromItem for End<L> {
    type Item = L;
    fn new(item: Self::Item) -> Self {
        Self::Bare(item)
    }
}

impl<L> From<Ace<L>> for End<L> {
    fn from(value: Ace<L>) -> Self {
        Self::Link(value)
    }
}

impl<L> From<&Ace<L>> for End<L> {
    fn from(value: &Ace<L>) -> Self {
        Self::Link(value.clone())
    }
}

impl<'a> From<&'a str> for End<String> {
    fn from(value: &'a str) -> Self {
        Self::Bare(value.into())
    }
}

impl<L> ToLoad for End<L>
where
    L: Clone,
{
    type Load = L;
    fn load(&self) -> Self::Load {
        match self {
            Self::Bare(bare) => bare.clone(),
            Self::Link(ace) => ace.load(),
        }
    }
}

impl<L> Read for End<L>
where
    L: 'static + SendSync, // + Send + Sync,
{
    type Item = L;
    fn read<T, F: FnOnce(&L) -> T>(&self, read: F) -> T {
        match self {
            Self::Bare(bare) => read(bare),
            Self::Link(ace) => ace.read(read),
        }
    }
}

impl<L> Backed for End<L>
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

// impl<L> FromAce for End<L> {
//     type Load = L;
//     fn from_ace(ace: &Ace<L>) -> Self {
//         Self::Link(ace.clone())
//     }
// }

// impl<L> Grant for Ace<L>
// where
//     L: Clone,
// {
//     type Load = Ace<L>;
//     fn grant(&self) -> Ace<L> {
//         match self {
//             Self::Bare(bare) => bare.ace(),
//             Self::Link(ace) => ace.clone(),
//         }
//     }
// }

// /// View of ace view, by plan.
// pub type Plan<P, T, L> = View<role::Plan<P, T, Ace<L>>, Ace<L>>;

// impl<L> Solve for Ace<L>
// where
//     L: Clone,
// {
//     type Task = ();
//     type Load = Ace<L>;
//     fn solve(&self, task: Self::Task) -> Self::Load {
//         match self {
//             Self::Bare(bare) => bare.ace(),
//             Self::Link(ace) => ace.clone(),
//         }
//     }
// }

// impl<L> IntoView for End<L> {
//     type Item = L;
//     fn into_view(role: Self::Item) -> Self {
//         Self::Role(role)
//     }
// }
