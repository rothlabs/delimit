use crate::*;

pub use load::*;
//pub use task::*;
pub use sole::*;

mod load;
//mod task;
mod sole;

#[derive(Clone, Serialize)]
pub enum View<R, B> {
    Role(R),
    Base(B),
}

impl<R, B> Default for View<R, B>
where
    B: Default,
{
    fn default() -> Self {
        Self::Base(B::default())
    }
}

impl<R, B> IntoView for View<R, B> {
    type Item = R;
    fn into_view(role: Self::Item) -> Self {
        Self::Role(role)
    }
}

impl<R, B> Reader for View<R, B>
where
    R: Grant,
    R::Load: Reader<Unit = B::Unit>,
    B: Reader + Send + Sync + 'static,
{
    type Unit = B::Unit;
    fn reader<F: FnOnce(&Self::Unit)>(&self, read: F) {
        match self {
            Self::Role(role) => role.grant().reader(read),
            Self::Base(bare) => bare.reader(read),
        };
    }
}

impl<R, B> Grant for View<R, B>
where
    R: Grant,
    B: Clone + IntoView<Item = R::Load>,
{
    type Load = B;
    fn grant(&self) -> Self::Load {
        match self {
            Self::Role(role) => B::into_view(role.grant()),
            Self::Base(load) => load.clone(),
        }
    }
}

impl<R, B> Backed for View<R, B>
where
    R: Backed<Back = Back>,
    B: Backed<Back = Back>,
{
    type Back = Back;
    fn backed(&self, root: &Self::Back) -> Self {
        match self {
            Self::Role(role) => View::Role(role.backed(root)),
            Self::Base(item) => View::Base(item.backed(root)),
        }
    }
}

pub trait AddView {
    type View;
    fn add_view(&mut self, view: Self::View);
}

impl<R, B> AddView for Vec<View<R, B>> {
    type View = View<R, B>;
    fn add_view(&mut self, view: Self::View) {
        self.push(view);
    }
}

pub trait AddRole {
    type Item;
    fn add_role(&mut self, role: Self::Item);
}

impl<R, B> AddRole for Vec<View<R, B>> {
    type Item = R;
    fn add_role(&mut self, role: Self::Item) {
        self.push(View::Role(role));
    }
}

pub trait AddSole {
    type Load;
    fn add_sole(&mut self, sole: Sole<Self::Load>);
}

impl<R, B> AddSole for Vec<View<R, B>>
where
    B: FromSole<B>,
{
    type Load = B;
    fn add_sole(&mut self, sole: Sole<B>) {
        self.push(View::Base(B::from_sole(sole)))
    }
}

pub trait AddItem {
    type Item;
    fn add_item<T: Grant<Load = Self::Item>>(&mut self, item: &T);
}

impl<R, B> AddItem for Vec<View<R, B>> {
    type Item = View<R, B>;
    fn add_item<T: Grant<Load = Self::Item>>(&mut self, item: &T) {
        self.push(item.grant());
    }
}

pub trait ToViewsBuilder<'a, R, B> {
    fn back(&'a mut self, root: &'a Back) -> ViewsBuilder<R, B>;
}

impl<'a, R, B> ToViewsBuilder<'a, R, B> for Vec<View<R, B>> {
    fn back(&'a mut self, back: &'a Back) -> ViewsBuilder<R, B> {
        ViewsBuilder { views: self, back }
    }
}

pub struct ViewsBuilder<'a, R, B> {
    views: &'a mut Vec<View<R, B>>,
    back: &'a Back,
}

impl<'a, R, B> ViewsBuilder<'a, R, B>
where
    R: Backed<Back = Back>,
    B: Backed<Back = Back>,
{
    pub fn add_view(&mut self, view: &View<R, B>) -> &mut Self {
        self.views.add_view(view.backed(self.back));
        self
    }
}

impl<'a, R, B> ViewsBuilder<'a, R, B>
where
    R: Backed<Back = Back>,
{
    pub fn add_role(&mut self, role: &R) -> &mut Self {
        self.views.add_role(role.backed(self.back));
        self
    }
}

impl<'a, R, B> ViewsBuilder<'a, R, B>
where
    B: Backed<Back = Back> + FromSole<B>,
{
    pub fn add_sole(&mut self, sole: &Sole<B>) -> &mut Self {
        self.views.add_sole(sole.backed(self.back));
        self
    }
}

impl<'a, R, B> ViewsBuilder<'a, R, B> {
    fn add_item<T: Grant<Load = View<R, B>> + Backed<Back = Back>>(&mut self, item: &T) -> &mut Self  {
        self.views.add_item(&item.backed(self.back));
        self
    }
}




// pub trait AddRole {
//     type Actual;
//     type Method;
//     fn add_role(&mut self, role: Role<Self::Actual, Self::Method>);
// }

// impl<R, L> AddRole for Vec<View<R, L>> 
// where 
//     R: 
// {
//     type Actual = A;
//     type Method = Ploy<L>;
//     fn add_role(&mut self, role: Role<Self::Actual, Self::Method>) {
//         self.push(View::Role(role));
//     }
// }
