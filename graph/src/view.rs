use crate::*;

pub use base::*;
pub use sole::*;

mod base;
mod sole;

#[derive(Clone)]
pub enum View<I, R, E> {
    Item(I),
    Role(Role<R, E>),
}

impl<I, R, E> Solve for View<I, R, E>
where
    I: Clone + IntoRole<Load = R>,
{
    type Load = I;
    fn solve(&self) -> Self::Load {
        match self {
            View::Item(item) => item.clone(),
            View::Role(role) => I::into_role(role.solve()),
        }
    }
}

impl<I, R, E> WithRoot for View<I, R, E>
where
    I: WithRoot<Root = Root>,
    E: Clone,
{
    type Root = Root;
    fn with_root(&self, root: &Self::Root) -> Self {
        match self {
            View::Item(item) => View::Item(item.with_root(root)),
            View::Role(role) => View::Role(role.with_root(root)),
        }
    }
}

pub trait AddToViews {
    type View;
    // fn add_item<T: Solve<Load = Self::View>>(&mut self, item: &T);
    fn add_view(&mut self, view: &Self::View);
}

impl<I, R, E> AddToViews for Vec<View<I, R, E>>
where
    I: Clone,
    R: Clone,
    E: Clone,
{
    type View = View<I, R, E>;
    // fn add_item<T: Solve<Load = Self::View>>(&mut self, item: &T) {
    //     self.push(item.solve());
    // }
    fn add_view(&mut self, view: &Self::View) {
        self.push(view.clone());
    }
}

pub trait ToViewsBuilder<'a, I, R, E> {
    fn root(&'a mut self, reactor: &'a Root) -> ViewsBuilder<I, R, E>;
}

impl<'a, I, R, E> ToViewsBuilder<'a, I, R, E> for Vec<View<I, R, E>> {
    fn root(&'a mut self, root: &'a Root) -> ViewsBuilder<I, R, E> {
        ViewsBuilder { views: self, root }
    }
}

pub struct ViewsBuilder<'a, I, R, E> {
    views: &'a mut Vec<View<I, R, E>>,
    root: &'a Root,
}

impl<'a, I, R, E> ViewsBuilder<'a, I, R, E>
where
    I: Clone + WithRoot<Root = Root>,
    R: Clone,
    E: Clone,
{
    pub fn add_view(&mut self, view: &View<I, R, E>) -> &mut Self {
        self.views.push(view.with_root(self.root));
        self
    }
}

/////////////////////////////////////////////////

// impl<I, R, E> SolveWithRoot for View<I, R, E>
// where
//     I: WithRoot<Root = Root> + IntoRole<Load = R>,
//     R: WithRoot<Root = Root>,
// {
//     type Load = I;
//     fn solve_with_root(&self, root: &Root) -> Self::Load {
//         match self {
//             View::Item(item) => item.with_root(root),
//             View::Role(role) => I::into_role(role.solve().with_root(root)),
//         }
//     }
// }
