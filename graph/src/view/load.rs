use crate::*;

#[derive(Clone, Serialize)]
pub enum PloyView<L, E> {
    Bare(L),
    Sole(Sole<L>),
    Role(Role<Sole<L>, E>),
}

impl<L, E> IntoRole for PloyView<L, E> {
    type Load = Role<Sole<L>, E>;
    fn into_role(load: Self::Load) -> Self {
        Self::Role(load)
    }
}

impl<L, E> Default for PloyView<L, E>
where
    L: Default,
{
    fn default() -> Self {
        Self::Bare(L::default())
    }
}

impl<L, E> Reader for PloyView<L, E>
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
impl<L, E> Grant for PloyView<L, E>
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

impl<L, E> WithRoot for PloyView<L, E>
where
    L: Clone,
    E: Clone,
{
    type Root = Back;
    fn with_root(&self, root: &Self::Root) -> Self {
        match self {
            PloyView::Bare(bare) => PloyView::Bare(bare.clone()),
            PloyView::Sole(leaf) => PloyView::Sole(leaf.with_root(root)),
            PloyView::Role(role) => PloyView::Role(role.with_root(root)),
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
    fn add_role(&mut self, role: Role<Sole<Self::Load>, Self::Exact>);
}

impl<L, E> AddToLoadViews for Vec<PloyView<L, E>>
where
    L: Clone + 'static,
    E: Clone,
{
    type View = PloyView<L, E>;
    type Load = L;
    type Exact = E;
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
    fn add_role(&mut self, role: Role<Sole<L>, E>) {
        self.push(PloyView::Role(role));
    }
}

pub trait AddStr {
    fn add_str(&mut self, load: &str) -> &mut Self;
}

impl<E> AddStr for Vec<PloyView<String, E>> {
    fn add_str(&mut self, load: &str) -> &mut Self {
        self.push(PloyView::Bare(load.into()));
        self
    }
}

pub trait ToLoadViewsBuilder<'a, L, E> {
    fn root(&'a mut self, root: &'a Back) -> LoadViewsBuilder<L, E>;
}

impl<'a, L, E> ToLoadViewsBuilder<'a, L, E> for Vec<PloyView<L, E>> {
    fn root(&'a mut self, root: &'a Back) -> LoadViewsBuilder<L, E> {
        LoadViewsBuilder { views: self, root }
    }
}

pub struct LoadViewsBuilder<'a, L, E> {
    views: &'a mut Vec<PloyView<L, E>>,
    root: &'a Back,
}

impl<'a, L, E> LoadViewsBuilder<'a, L, E>
where
    L: Clone + 'static,
    E: Clone,
{
    pub fn add_item<T: Grant<Load = PloyView<L, E>> + WithRoot<Root = Back>>(
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

impl<'a, E> AddStr for LoadViewsBuilder<'a, String, E> {
    fn add_str(&mut self, load: &str) -> &mut Self {
        self.views.push(PloyView::Bare(load.into()));
        self
    }
}
