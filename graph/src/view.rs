use crate::*;

pub use end::End;

pub mod end;

/// A base or a role that must provide a base via granting or solving.
/// Views are phrased as "view of BASE with ROLE" or "BASE view with ROLE".
/// The base could be another view, allowing for a chain of views.
/// Common use-case: Some unit field could link to same logic level or it could
/// link to logic of a sub-graph load.
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

impl<'a, R, B, L> From<&'a L> for View<R, B>
where
    &'a L: Into<B>
{
    fn from(value: &'a L) -> Self {
        Self::Base(value.into())
    }
}

impl<R, B, L> From<Ace<L>> for View<R, B>
where
    Ace<L>: Into<B>,
{
    fn from(value: Ace<L>) -> Self {
        Self::Base(value.into())
    }
}

impl<'a, R, B> From<&'a str> for View<R, B>
where
    &'a str: Into<B>,
{
    fn from(value: &'a str) -> Self {
        Self::Base(value.into())
    }
}

impl<R, B> FromItem for View<R, B>
where
    B: FromItem,
{
    type Item = B::Item;
    fn new(item: Self::Item) -> Self {
        Self::Base(B::new(item))
    }
}

impl<R, B> IntoView for View<R, B> {
    type Item = R;
    fn into_view(role: Self::Item) -> Self {
        Self::Role(role)
    }
}

impl<R, B> ToLoad for View<R, B>
where
    R: Grant,
    R::Load: ToLoad<Load = B::Load>,
    B: ToLoad,
{
    type Load = B::Load;
    fn load(&self) -> Self::Load {
        match self {
            Self::Base(base) => base.load(),
            Self::Role(role) => role.grant().load(),
        }
    }
}

impl<R, B> ToLoadByTask for View<R, B>
where
    R: Solve,
    R::Load: ToLoad<Load = B::Load>,
    B: ToLoad,
{
    type Task = R::Task;
    type Load = B::Load;
    fn load(&self, task: Self::Task) -> Self::Load {
        match self {
            Self::Base(bare) => bare.load(),
            Self::Role(role) => role.solve(task).load(),
        }
    }
}

impl<R, B> Read for View<R, B>
where
    R: Grant,
    R::Load: Read<Item = B::Item>,
    B: Read,
{
    type Item = B::Item;
    fn read<T, F: FnOnce(&Self::Item) -> T>(&self, read: F) -> T {
        match self {
            Self::Role(role) => role.grant().read(read),
            Self::Base(bare) => bare.read(read),
        }
    }
}

impl<R, B> ReadByTask for View<R, B>
where
    R: Solve,
    R::Load: Read<Item = B::Item>,
    B: Read,
{
    type Task = R::Task;
    type Item = B::Item;
    fn read<T, F: FnOnce(&Self::Item) -> T>(&self, task: Self::Task, read: F) -> T {
        match self {
            Self::Role(role) => role.solve(task).read(read),
            Self::Base(bare) => bare.read(read),
        }
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
            Self::Base(base) => base.clone(),
        }
    }
}

impl<R, B> Solve for View<R, B>
where
    R: Solve,
    B: Clone + IntoView<Item = R::Load>,
{
    type Task = R::Task;
    type Load = B;
    fn solve(&self, task: Self::Task) -> Self::Load {
        match self {
            Self::Role(role) => B::into_view(role.solve(task)),
            Self::Base(base) => base.clone(),
        }
    }
}

impl<R, B> Serve for View<R, B>
where
    R: Serve,
    B: Clone + IntoView<Item = R::Load>,
{
    type Task = R::Task;
    type Load = B;
    fn serve(&self, task: Self::Task) -> Self::Load {
        match self {
            Self::Role(role) => B::into_view(role.serve(task)),
            Self::Base(base) => base.clone(),
        }
    }
}

impl<R, B> Backed for View<R, B>
where
    R: Backed,
    B: Backed,
{
    fn backed(&self, back: &Back) -> Self {
        match self {
            Self::Role(role) => View::Role(role.backed(back)),
            Self::Base(item) => View::Base(item.backed(back)),
        }
    }
}

pub trait AddRole {
    type Role;
    fn role(&mut self, role: Self::Role) -> &mut Self;
}

pub trait AddBase<T> {
    fn base(&mut self, item: T) -> &mut Self;
}

impl<R, B> AddRole for Vec<View<R, B>> {
    type Role = R;
    fn role(&mut self, role: Self::Role) -> &mut Self {
        self.push(View::Role(role));
        self
    }
}

impl<R, B, L> AddBase<L> for Vec<View<R, B>>
where
    L: Into<View<R, B>>,
{
    fn base(&mut self, item: L) -> &mut Self {
        self.push(item.into());
        self
    }
}

pub trait ToViewsMutator<'a, R, B> {
    fn back(&'a mut self, back: &'a Back) -> ViewsMutator<R, B>;
}

impl<'a, R, B> ToViewsMutator<'a, R, B> for Vec<View<R, B>> {
    fn back(&'a mut self, back: &'a Back) -> ViewsMutator<R, B> {
        ViewsMutator { views: self, back }
    }
}

pub struct ViewsMutator<'a, R, B> {
    views: &'a mut Vec<View<R, B>>,
    back: &'a Back,
}

impl<'a, R, B> ViewsMutator<'a, R, B>
where
    R: Backed,
    B: Backed,
{
    pub fn view(&mut self, view: &View<R, B>) -> &mut Self {
        self.views.push(view.backed(self.back));
        self
    }
}

impl<'a, R, B> ViewsMutator<'a, R, B>
where
    R: Backed,
{
    pub fn role(&mut self, role: &R) -> &mut Self {
        self.views.role(role.backed(self.back));
        self
    }
}

impl<'a, R, B, L: 'a> AddBase<&'a L> for ViewsMutator<'a, R, B>
where
    L: Backed + Into<View<R, B>>,
{
    fn base(&mut self, item: &'a L) -> &mut Self {
        self.views.base(item.backed(self.back));
        self
    }
}

impl<'a, R, B> ViewsMutator<'a, R, B>
where
    &'a str: Into<B>,
{
    pub fn str(&mut self, str: &'a str) -> &mut Self {
        self.views.base(str);
        self
    }
}

pub struct ViewsBuilder<'a, R, B> {
    views: Vec<View<R, B>>,
    back: &'a Back,
}

impl<'a, R, B> ViewsBuilder<'a, R, B>
where
    R: Clone,
    B: Clone,
{
    pub fn new(back: &'a Back) -> Self {
        Self {
            views: vec![],
            back,
        }
    }
    pub fn build(&self) -> Vec<View<R, B>> {
        self.views.clone()
    }
}

impl<'a, R, B> ViewsBuilder<'a, R, B>
where
    R: Backed,
    B: Backed,
{
    pub fn view(&mut self, view: &View<R, B>) -> &mut Self {
        self.views.push(view.backed(self.back));
        self
    }
}

impl<'a, R, B> ViewsBuilder<'a, R, B>
where
    R: Backed,
{
    pub fn role(&mut self, role: &R) -> &mut Self {
        self.views.role(role.backed(self.back));
        self
    }
}

impl<'a, R, B, L: 'a> AddBase<&'a L> for ViewsBuilder<'a, R, B>
where
    L: Backed + Into<View<R, B>>,
{
    fn base(&mut self, item: &'a L) -> &mut Self {
        self.views.base(item.backed(self.back));
        self
    }
}

impl<'a, R, B> ViewsBuilder<'a, R, B>
where
    &'a str: Into<B>,
{
    pub fn str(&mut self, str: &'a str) -> &mut Self {
        self.views.base(str);
        self
    }
}









// pub trait AddBase<T> {
//     fn add(&mut self, item: T) -> &mut Self;
// }


// impl<'a, R, B, L: 'a> AddBase<&'a L> for ViewsMutator<'a, R, B>
// where
//     L: Clone + Into<View<R, B>>,
// {
//     fn add(&mut self, item: &'a L) -> &mut Self {
//         self.views.base(item.clone());
//         self
//     }
// }



// impl<'a, R, B, L> AddItem<&'a L> for ViewsBuilder<'a, R, B>
// where
//     L: 'a,// + Clone,
//     View<R, B>: From<&'a L>,
// {
//     fn add_item(&mut self, item: &'a L) -> &mut Self {
//         self.views.add_item(item);
//         self
//     }
// }

// impl<R, B> FromAce for View<R, B>
// where
//     B: FromAce,
// {
//     type Load = B::Load;
//     fn from_ace(ace: &Ace<Self::Load>) -> Self {
//         Self::Base(B::from_ace(ace))
//     }
// }

// impl<'a, R, B> ViewsMutator<'a, R, B>
// where
//     B: Backed + FromAce,
// {
//     pub fn add_ace(&mut self, ace: &link::Ace<B::Load>) -> &mut Self {
//         self.views.add_ace(ace.backed(self.back));
//         self
//     }
// }

// impl<'a, R, B> ViewsBuilder<'a, R, B> {
//     pub fn use_ploy<T: Grant<Load = View<R, B>> + Backed>(&mut self, item: &T) -> &mut Self {
//         self.views.use_ploy(&item.backed(self.back));
//         self
//     }
// }

// impl<'a, R, B> ViewsBuilder<'a, R, B>
// where
//     B: Backed + FromAce,
// {
//     pub fn add_ace(&mut self, ace: &link::Ace<B::Load>) -> &mut Self {
//         self.views.add_ace(ace.backed(self.back));
//         self
//     }
// }

// impl<'a, R, B> ViewsBuilder<'a, R, B> {
//     pub fn use_ploy<T: Grant<Load = View<R, B>> + Backed>(&mut self, item: &T) -> &mut Self {
//         self.views.use_ploy(&item.backed(self.back));
//         self
//     }
// }

// impl<R, B> AddAce for Vec<View<R, B>>
// where
//     B: FromAce,
// {
//     type Load = B::Load;
//     fn add_ace(&mut self, ace: link::Ace<B::Load>) {
//         self.push(View::Base(B::from_ace(&ace)))
//     }
// }

// impl<R, B> UsePloy for Vec<View<R, B>> {
//     type Load = View<R, B>;
//     fn use_ploy<T: Grant<Load = Self::Load> + ?Sized>(&mut self, item: &T) {
//         self.push(item.grant());
//     }
// }

// impl<R, B> AddStr for Vec<View<R, B>>
// where
//     B: From<&'static str>,
// {
//     fn add_str(&mut self, str: &'static str) {
//         self.push(View::Base(B::from(str)));
//     }
// }
