use crate::*;

#[derive(Clone, Serialize)]
pub enum LoadView<L, E> {
    Bare(L),
    Sole(Sole<L>),
    Role(PloyRole<Sole<L>, E>),
}

impl<L, E> IntoRole for LoadView<L, E> {
    type Load = PloyRole<Sole<L>, E>;
    fn into_role(load: Self::Load) -> Self {
        Self::Role(load)
    }
}

impl<L, E> Default for LoadView<L, E>
where
    L: Default,
{
    fn default() -> Self {
        Self::Bare(L::default())
    }
}

impl<L, E> Reader for LoadView<L, E>
where
    L: 'static + Send + Sync,
{
    type Unit = L;
    fn reader<F: FnOnce(&L)>(&self, read: F) {
        match self {
            LoadView::Bare(bare) => read(bare),
            LoadView::Sole(sole) => sole.reader(read),
            LoadView::Role(role) => role.grant().reader(read),
        };
    }
}

// it is creating a new Sole on each grant if bare. Is this bad?
impl<L, E> Grant for LoadView<L, E>
where
    L: Clone + 'static,
{
    type Load = Sole<L>;
    fn grant(&self) -> Sole<L> {
        match self {
            LoadView::Bare(bare) => bare.clone().into_sole(),
            LoadView::Sole(leaf) => leaf.clone(),
            LoadView::Role(role) => role.grant(),
        }
    }
}

impl<L, E> WithRoot for LoadView<L, E>
where
    L: Clone,
    E: Clone,
{
    type Root = Root;
    fn with_root(&self, root: &Self::Root) -> Self {
        match self {
            LoadView::Bare(bare) => LoadView::Bare(bare.clone()),
            LoadView::Sole(leaf) => LoadView::Sole(leaf.with_root(root)),
            LoadView::Role(role) => LoadView::Role(role.with_root(root)),
        }
    }
}

pub trait AddToLoadViews {
    type View;
    type Load;
    type Exact;
    fn add_item<T: Grant<Load = Self::View>>(&mut self, item: &T);
    // fn add_view(&mut self, view: Self::View);
    fn add_bare(&mut self, bare: &Self::Load);
    fn add_leaf(&mut self, leaf: Sole<Self::Load>);
    fn add_role(&mut self, role: PloyRole<Sole<Self::Load>, Self::Exact>);
}

impl<L, E> AddToLoadViews for Vec<LoadView<L, E>>
where
    L: Clone + 'static,
    E: Clone,
{
    type View = LoadView<L, E>;
    type Load = L;
    type Exact = E;
    fn add_item<T: Grant<Load = Self::View>>(&mut self, item: &T) {
        self.push(item.grant());
    }
    // fn add_view(&mut self, item: Self::View) {
    //     self.push(item);
    // }
    fn add_bare(&mut self, bare: &L) {
        self.push(LoadView::Bare(bare.clone()));
    }
    fn add_leaf(&mut self, leaf: Sole<L>) {
        self.push(LoadView::Sole(leaf));
    }
    fn add_role(&mut self, role: PloyRole<Sole<L>, E>) {
        self.push(LoadView::Role(role));
    }
}

pub trait AddStr {
    fn add_str(&mut self, load: &str) -> &mut Self;
}

impl<E> AddStr for Vec<LoadView<String, E>> {
    fn add_str(&mut self, load: &str) -> &mut Self {
        self.push(LoadView::Bare(load.into()));
        self
    }
}

pub trait ToLoadViewsBuilder<'a, L, E> {
    fn root(&'a mut self, reactor: &'a Root) -> LoadViewsBuilder<L, E>;
}

impl<'a, L, E> ToLoadViewsBuilder<'a, L, E> for Vec<LoadView<L, E>> {
    fn root(&'a mut self, root: &'a Root) -> LoadViewsBuilder<L, E> {
        LoadViewsBuilder { views: self, root }
    }
}

pub struct LoadViewsBuilder<'a, L, E> {
    views: &'a mut Vec<LoadView<L, E>>,
    root: &'a Root,
}

impl<'a, L, E> LoadViewsBuilder<'a, L, E>
where
    L: Clone + 'static,
    E: Clone,
{
    pub fn add_item<T: Grant<Load = LoadView<L, E>> + WithRoot<Root = Root>>(
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
    pub fn add_role(&mut self, role: &PloyRole<Sole<L>, E>) -> &mut Self {
        self.views.add_role(role.with_root(self.root));
        self
    }
}

impl<'a, E> AddStr for LoadViewsBuilder<'a, String, E> {
    fn add_str(&mut self, load: &str) -> &mut Self {
        self.views.push(LoadView::Bare(load.into()));
        self
    }
}
