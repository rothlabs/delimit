use crate::*;

#[derive(Clone, Serialize)]
pub enum BaseView<L, E> {
    Bare(L),
    Sole(Sole<L>),
    Role(Role<Sole<L>, E>),
}

impl<L, E> IntoRole for BaseView<L, E> {
    type Load = Role<Sole<L>, E>;
    fn into_role(load: Self::Load) -> Self {
        Self::Role(load)
    }
}

impl<L, E> Default for BaseView<L, E>
where
    L: Default,
{
    fn default() -> Self {
        Self::Bare(L::default())
    }
}

impl<L, E> Reader for BaseView<L, E>
where
    L: 'static,
{
    type Unit = L;
    fn reader<F: FnOnce(&L)>(&self, read: F) {
        match self {
            BaseView::Bare(bare) => read(bare),
            BaseView::Sole(sole) => sole.reader(read),
            BaseView::Role(role) => role.solve().reader(read),
        };
    }
}

// it is creating a new leaf on each solve if bare. Is this bad?
impl<L, E> Solve for BaseView<L, E>
where
    L: Clone + 'static,
{
    type Load = Sole<L>;
    fn solve(&self) -> Sole<L> {
        match self {
            BaseView::Bare(bare) => bare.clone().into_sole(),
            BaseView::Sole(leaf) => leaf.clone(),
            BaseView::Role(role) => role.solve(),
        }
    }
}

impl<L, E> WithRoot for BaseView<L, E>
where
    L: Clone,
    E: Clone,
{
    type Root = Root;
    fn with_root(&self, root: &Self::Root) -> Self {
        match self {
            BaseView::Bare(bare) => BaseView::Bare(bare.clone()),
            BaseView::Sole(leaf) => BaseView::Sole(leaf.with_root(root)),
            BaseView::Role(role) => BaseView::Role(role.with_root(root)),
        }
    }
}

pub trait AddToBaseViews {
    type View;
    type Load;
    type Exact;
    fn add_item<T: Solve<Load = Self::View>>(&mut self, item: &T);
    // fn add_view(&mut self, view: Self::View);
    fn add_bare(&mut self, bare: &Self::Load);
    fn add_leaf(&mut self, leaf: Sole<Self::Load>);
    fn add_role(&mut self, role: Role<Sole<Self::Load>, Self::Exact>);
}

impl<L, E> AddToBaseViews for Vec<BaseView<L, E>>
where
    L: Clone + 'static,
    E: Clone,
{
    type View = BaseView<L, E>;
    type Load = L;
    type Exact = E;
    fn add_item<T: Solve<Load = Self::View>>(&mut self, item: &T) {
        self.push(item.solve());
    }
    // fn add_view(&mut self, item: Self::View) {
    //     self.push(item);
    // }
    fn add_bare(&mut self, bare: &L) {
        self.push(BaseView::Bare(bare.clone()));
    }
    fn add_leaf(&mut self, leaf: Sole<L>) {
        self.push(BaseView::Sole(leaf));
    }
    fn add_role(&mut self, role: Role<Sole<L>, E>) {
        self.push(BaseView::Role(role));
    }
}

pub trait AddStr {
    fn add_str(&mut self, load: &str) -> &mut Self;
}

impl<E> AddStr for Vec<BaseView<String, E>> {
    fn add_str(&mut self, load: &str) -> &mut Self {
        self.push(BaseView::Bare(load.into()));
        self
    }
}

pub trait ToBaseViewsBuilder<'a, L, E> {
    fn root(&'a mut self, reactor: &'a Root) -> BaseViewsBuilder<L, E>;
}

impl<'a, L, E> ToBaseViewsBuilder<'a, L, E> for Vec<BaseView<L, E>> {
    fn root(&'a mut self, root: &'a Root) -> BaseViewsBuilder<L, E> {
        BaseViewsBuilder { views: self, root }
    }
}

pub struct BaseViewsBuilder<'a, L, E> {
    views: &'a mut Vec<BaseView<L, E>>,
    root: &'a Root,
}

impl<'a, L, E> BaseViewsBuilder<'a, L, E>
where
    L: Clone + 'static,
    E: Clone,
{
    pub fn add_item<T: Solve<Load = BaseView<L, E>> + WithRoot<Root = Root>>(
        &mut self,
        item: &T,
    ) -> &mut Self {
        self.views.add_item(&item.with_root(self.root));
        self
    }
    // pub fn add_view(&mut self, view: &LeafView<L, E>) -> &mut Self {
    //     self.views.add_view(view.with_root(self.root));
    //     self
    // }
    pub fn add_bare(&mut self, bare: &L) -> &mut Self {
        self.views.add_bare(bare);
        self
    }
    pub fn add_leaf(&mut self, leaf: &Sole<L>) -> &mut Self {
        self.views.add_leaf(leaf.with_root(self.root));
        self
    }
    pub fn add_role(&mut self, role: &Role<Sole<L>, E>) -> &mut Self {
        self.views.add_role(role.with_root(self.root));
        self
    }
}

impl<'a, E> AddStr for BaseViewsBuilder<'a, String, E> {
    fn add_str(&mut self, load: &str) -> &mut Self {
        self.views.push(BaseView::Bare(load.into()));
        self
    }
}
