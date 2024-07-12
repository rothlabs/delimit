use crate::*;

pub use load::*;
pub use task::*;
pub use sole::*;

mod load;
mod task;
mod sole;

#[derive(Clone, Serialize)]
pub enum View<I, L, E> {
    Item(I),
    Role(PloyRole<L, E>),
}

impl<I, L, E> Grant for View<I, L, E>
where
    I: Clone + IntoRole<Load = L>,
{
    type Load = I;
    fn grant(&self) -> Self::Load {
        match self {
            View::Item(item) => item.clone(),
            View::Role(role) => I::into_role(role.grant()),
        }
    }
}

impl<I, L, E> WithRoot for View<I, L, E>
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
    fn root(&'a mut self, root: &'a Root) -> ViewsBuilder<I, L, E>;
}

impl<'a, I, R, E> ToViewsBuilder<'a, I, R, E> for Vec<View<I, R, E>> {
    fn root(&'a mut self, root: &'a Root) -> ViewsBuilder<I, R, E> {
        ViewsBuilder { views: self, root }
    }
}

pub struct ViewsBuilder<'a, I, L, E> {
    views: &'a mut Vec<View<I, L, E>>,
    root: &'a Root,
}

impl<'a, I, L, E> ViewsBuilder<'a, I, L, E>
where
    I: Clone + WithRoot<Root = Root>,
    L: Clone,
    E: Clone,
{
    pub fn add_view(&mut self, view: &View<I, L, E>) -> &mut Self {
        self.views.push(view.with_root(self.root));
        self
    }
}

