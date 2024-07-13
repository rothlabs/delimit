use crate::*;

pub use load::*;
pub use task::*;
pub use sole::*;

mod load;
mod task;
mod sole;

#[derive(Clone, Serialize)]
pub enum View<A, M, L> {
    Bare(L),
    Role(Role<A, M>), 
}

impl<A, M, L> Grant for View<A, M, L>
where
    A: Clone,// + IntoRole<Load = L>,
    M: Grant<Load = L>,
    L: Clone + IntoRole<Load = L>,
{
    type Load = L;
    fn grant(&self) -> Self::Load {
        match self {
            Self::Bare(load) => load.clone(),
            Self::Role(role) => L::into_role(role.grant()),
        }
    }
}

impl<I, L, A> WithRoot for View<I, L, A>
where
    I: WithRoot<Root = Back>,
    A: WithRoot<Root = Back>,
    L: Clone,
{
    type Root = A::Root;
    fn with_root(&self, root: &Self::Root) -> Self {
        match self {
            View::Bare(item) => View::Bare(item.with_root(root)),
            View::Role(role) => View::Role(role.with_root(root)),
        }
    }
}

pub trait AddToViews {
    type View;
    // fn add_item<T: Grant<Load = Self::View>>(&mut self, item: &T);
    fn add_view(&mut self, view: &Self::View);
}

impl<I, L, E> AddToViews for Vec<View<I, L, E>>
where
    I: Clone,
    L: Clone,
    E: Clone,
{
    type View = View<I, L, E>;
    // fn add_item<T: Grant<Load = Self::View>>(&mut self, item: &T) {
    //     self.push(item.grant());
    // }
    fn add_view(&mut self, view: &Self::View) {
        self.push(view.clone());
    }
}

pub trait ToViewsBuilder<'a, I, L, E> {
    fn root(&'a mut self, root: &'a Back) -> ViewsBuilder<I, L, E>;
}

impl<'a, I, R, E> ToViewsBuilder<'a, I, R, E> for Vec<View<I, R, E>> {
    fn root(&'a mut self, root: &'a Back) -> ViewsBuilder<I, R, E> {
        ViewsBuilder { views: self, root }
    }
}

pub struct ViewsBuilder<'a, I, L, E> {
    views: &'a mut Vec<View<I, L, E>>,
    root: &'a Back,
}

impl<'a, I, L, E> ViewsBuilder<'a, I, L, E>
where
    I: Clone + WithRoot<Root = Back>,
    L: Clone,
    E: Clone,
{
    pub fn add_view(&mut self, view: &View<I, L, E>) -> &mut Self {
        self.views.push(view.with_root(self.root));
        self
    }
}

