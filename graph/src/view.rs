use crate::*;

pub use ace_view::AceView;

mod ace_view;

// ploy ace bare
pub type Ploy<A, L> = View<role::Ploy<A, Ace<L>>, AceView<L>>;

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
    R::Load: Reader<Item = B::Item>,
    B: Reader + Send + Sync + 'static,
{
    type Item = B::Item;
    fn read<F: FnOnce(&Self::Item)>(&self, read: F) {
        match self {
            Self::Role(role) => role.grant().read(read),
            Self::Base(bare) => bare.read(read),
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
            Self::Base(base) => base.clone(),
        }
    }
}

impl<R, B> Backed for View<R, B>
where
    R: Backed,
    B: Backed,
{
    fn backed(&self, root: &Back) -> Self {
        match self {
            Self::Role(role) => View::Role(role.backed(root)),
            Self::Base(item) => View::Base(item.backed(root)),
        }
    }
}

pub trait AddRole {
    type Role;
    fn add_role(&mut self, role: Self::Role);
}

impl<R, B> AddRole for Vec<View<R, B>> {
    type Role = R;
    fn add_role(&mut self, role: Self::Role) {
        self.push(View::Role(role));
    }
}

impl<R, B> AddAce for Vec<View<R, B>>
where
    B: FromAce,
{
    type Load = B::Load;
    fn add_ace(&mut self, ace: Ace<B::Load>) {
        self.push(View::Base(B::from_ace(ace)))
    }
}

impl<R, B> UsePloy for Vec<View<R, B>> {
    type Load = View<R, B>;
    fn use_ploy<T: Grant<Load = Self::Load> + ?Sized>(&mut self, item: &T) {
        self.push(item.grant());
    }
}

impl<R, B> AddStr for Vec<View<R, B>>
where
    B: From<&'static str>,
{
    fn add_str(&mut self, str: &'static str) {
        self.push(View::Base(B::from(str)));
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
    R: Backed,
    B: Backed,
{
    pub fn push(&mut self, view: &View<R, B>) -> &mut Self {
        self.views.push(view.backed(self.back));
        self
    }
}

impl<'a, R, B> ViewsBuilder<'a, R, B>
where
    R: Backed,
{
    pub fn add_role(&mut self, role: &R) -> &mut Self {
        self.views.add_role(role.backed(self.back));
        self
    }
}

impl<'a, R, B> ViewsBuilder<'a, R, B>
where
    B: Backed + FromAce,
{
    pub fn add_ace(&mut self, ace: &Ace<B::Load>) -> &mut Self {
        self.views.add_ace(ace.backed(self.back));
        self
    }
}

impl<'a, R, B> ViewsBuilder<'a, R, B> {
    pub fn use_ploy<T: Grant<Load = View<R, B>> + Backed>(&mut self, item: &T) -> &mut Self {
        self.views.use_ploy(&item.backed(self.back));
        self
    }
}

impl<'a, R, B> ViewsBuilder<'a, R, B>
where
    B: From<&'static str>,
{
    pub fn add_str(&mut self, str: &'static str) -> &mut Self {
        self.views.add_str(str);
        self
    }
}
