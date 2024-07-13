use crate::*;

#[derive(Clone, Serialize)]
pub enum PloyView<A, L> {
    Bare(L),
    Sole(Sole<L>),
    Role(Role<A, Ploy<Sole<L>>>),
}

impl<A, L> IntoRole for PloyView<A, L> {
    type Load = Role<A, Ploy<Sole<L>>>;
    fn into_role(load: Self::Load) -> Self {
        Self::Role(load)
    }
}

impl<A, L> Default for PloyView<A, L>
where
    L: Default,
{
    fn default() -> Self {
        Self::Bare(L::default())
    }
}

impl<A, L> Reader for PloyView<A, L>
where
    L: 'static + Send + Sync,
{
    type Unit = L;
    fn reader<F: FnOnce(&L)>(&self, read: F) {
        match self {
            PloyView::Bare(bare) => read(bare),
            PloyView::Sole(sole) => sole.reader(read),
            PloyView::Role(role) => role.grant().reader(read),
        };
    }
}

// it is creating a new Sole on each grant if bare. Is this bad?
impl<A, L> Grant for PloyView<A, L>
where
    L: Clone + 'static,
{
    type Load = Sole<L>;
    fn grant(&self) -> Sole<L> {
        match self {
            PloyView::Bare(bare) => bare.clone().into_sole(),
            PloyView::Sole(leaf) => leaf.clone(),
            PloyView::Role(role) => role.grant(),
        }
    }
}

impl<A, L> WithRoot for PloyView<A, L>
where
    A: WithRoot<Root = Back>,
    L: Clone,
{
    type Root = Back;
    fn with_root(&self, root: &Self::Root) -> Self {
        match self {
            PloyView::Bare(bare) => PloyView::Bare(bare.clone()),
            PloyView::Sole(sole) => PloyView::Sole(sole.with_root(root)),
            PloyView::Role(role) => PloyView::Role(role.with_root(root)),
        }
    }
}

pub trait AddToLoadViews {
    type View;
    type Load;
    type Actual;
    fn add_item<T: Grant<Load = Self::View>>(&mut self, item: &T);
    // fn add_view(&mut self, view: Self::View);
    fn add_bare(&mut self, bare: &Self::Load);
    fn add_leaf(&mut self, leaf: Sole<Self::Load>);
    fn add_role(&mut self, role: Role<Self::Actual, Ploy<Sole<Self::Load>>>);
}

impl<A, L> AddToLoadViews for Vec<PloyView<A, L>>
where
    A: Clone,
    L: Clone + 'static,
{
    type View = PloyView<A, L>;
    type Actual = A;
    type Load = L;
    fn add_item<T: Grant<Load = Self::View>>(&mut self, item: &T) {
        self.push(item.grant());
    }
    // fn add_view(&mut self, item: Self::View) {
    //     self.push(item);
    // }
    fn add_bare(&mut self, bare: &L) {
        self.push(PloyView::Bare(bare.clone()));
    }
    fn add_leaf(&mut self, leaf: Sole<L>) {
        self.push(PloyView::Sole(leaf));
    }
    fn add_role(&mut self, role: Role<A, Ploy<Sole<L>>>) {
        self.push(PloyView::Role(role));
    }
}

pub trait AddStr {
    fn add_str(&mut self, load: &str) -> &mut Self;
}

impl<A> AddStr for Vec<PloyView<A, String>> {
    fn add_str(&mut self, load: &str) -> &mut Self {
        self.push(PloyView::Bare(load.into()));
        self
    }
}

pub trait ToLoadViewsBuilder<'a, A, L> {
    fn root(&'a mut self, root: &'a Back) -> LoadViewsBuilder<A, L>;
}

impl<'a, A, L> ToLoadViewsBuilder<'a, A, L> for Vec<PloyView<A, L>> {
    fn root(&'a mut self, root: &'a Back) -> LoadViewsBuilder<A, L> {
        LoadViewsBuilder { views: self, root }
    }
}

pub struct LoadViewsBuilder<'a, A, L> {
    views: &'a mut Vec<PloyView<A, L>>,
    root: &'a Back,
}

impl<'a, A, L> LoadViewsBuilder<'a, A, L>
where
    A: Clone + WithRoot<Root = Back>,
    L: Clone + 'static,
{
    pub fn add_item<T: Grant<Load = PloyView<A, L>> + WithRoot<Root = Back>>(
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
    pub fn add_role(&mut self, role: &Role<A, Ploy<Sole<L>>>) -> &mut Self {
        self.views.add_role(role.with_root(self.root));
        self
    }
}

impl<'a, A> AddStr for LoadViewsBuilder<'a, A, String> {
    fn add_str(&mut self, load: &str) -> &mut Self {
        self.views.push(PloyView::Bare(load.into()));
        self
    }
}
