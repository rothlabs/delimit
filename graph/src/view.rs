use crate::*;

pub use load::*;
//pub use task::*;
pub use sole::*;

mod load;
//mod task;
mod sole;

#[derive(Clone, Serialize)]
pub enum View<R, L> {
    Role(R),
    Bare(L),
}

impl<R, L> Grant for View<R, L>
where
    R: Grant,
    L: Clone + IntoView<Item = R::Load>,
{
    type Load = L;
    fn grant(&self) -> Self::Load {
        match self {
            Self::Role(role) => L::into_view(role.grant()),
            Self::Bare(load) => load.clone(),
        }
    }
}

impl<R, L> Backed for View<R, L>
where
    R: Backed<Back = Back>,
    L: Backed<Back = Back>,
{
    type Back = Back;
    fn backed(&self, root: &Self::Back) -> Self {
        match self {
            Self::Role(role) => View::Role(role.backed(root)),
            Self::Bare(item) => View::Bare(item.backed(root)),
        }
    }
}

pub trait AddToViews {
    type View;
    // fn add_item<T: Grant<Load = Self::View>>(&mut self, item: &T);
    fn add_view(&mut self, view: &Self::View);
}

impl<R, L> AddToViews for Vec<View<R, L>>
where
    R: Clone,
    L: Clone,
{
    type View = View<R, L>;
    // fn add_item<T: Grant<Load = Self::View>>(&mut self, item: &T) {
    //     self.push(item.grant());
    // }
    fn add_view(&mut self, view: &Self::View) {
        self.push(view.clone());
    }
}

pub trait ToViewsBuilder<'a, R, L> {
    fn root(&'a mut self, root: &'a Back) -> ViewsBuilder<R, L>;
}

impl<'a, R, L> ToViewsBuilder<'a, R, L> for Vec<View<R, L>> {
    fn root(&'a mut self, root: &'a Back) -> ViewsBuilder<R, L> {
        ViewsBuilder { views: self, root }
    }
}

pub struct ViewsBuilder<'a, R, L> {
    views: &'a mut Vec<View<R, L>>,
    root: &'a Back,
}

impl<'a, R, L> ViewsBuilder<'a, R, L>
where
    R: Backed<Back = Back>,
    L: Backed<Back = Back>,
{
    pub fn add_view(&mut self, view: &View<R, L>) -> &mut Self {
        self.views.push(view.backed(self.root));
        self
    }
}
