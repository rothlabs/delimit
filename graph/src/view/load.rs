use crate::*;

#[derive(Clone, Serialize)]
pub enum PloyView<A, L> {
    Bare(L),
    Sole(Sole<L>),
    Role(Role<A, Ploy<Sole<L>>>),
}

impl<A, L> IntoView for PloyView<A, L> {
    type Item = Role<A, Ploy<Sole<L>>>;
    fn into_view(load: Self::Item) -> Self {
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

impl<A, L> Backed for PloyView<A, L>
where
    A: Clone,
    L: Clone,
{
    type Back = Back;
    fn backed(&self, root: &Self::Back) -> Self {
        match self {
            PloyView::Bare(bare) => PloyView::Bare(bare.clone()),
            PloyView::Sole(sole) => PloyView::Sole(sole.backed(root)),
            PloyView::Role(role) => PloyView::Role(role.backed(root)),
        }
    }
}

pub trait AddToLoadViews {
    // type View;
    type Actual;
    type Load;
    type ItemLoad;
    // fn add_view(&mut self, view: Self::View);
    fn add_item<T: Grant<Load = Self::ItemLoad>>(&mut self, item: &T);
    fn add_bare(&mut self, bare: &Self::Load);
    fn add_leaf(&mut self, leaf: Sole<Self::Load>);
    fn add_role(&mut self, role: Role<Self::Actual, Ploy<Sole<Self::Load>>>);
}

impl<A, L> AddToLoadViews for Vec<PloyView<A, L>>
where
    A: Clone,
    L: Clone + 'static,
{
    // type View = PloyView<A, L>;
    type Actual = A;
    type Load = L;
    type ItemLoad = PloyView<A, L>; // Role<A, Ploy<Sole<L>>>; //
                                    // fn add_view(&mut self, view: Self::View) {
                                    //     self.push(view.clone());
                                    // }
    fn add_item<T: Grant<Load = Self::ItemLoad>>(&mut self, item: &T) {
        self.push(item.grant());
    }
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
    fn root(&'a mut self, back: &'a Back) -> LoadViewsBuilder<A, L> {
        LoadViewsBuilder { views: self, back }
    }
}

pub struct LoadViewsBuilder<'a, A, L> {
    views: &'a mut Vec<PloyView<A, L>>,
    back: &'a Back,
}

impl<'a, A, L> LoadViewsBuilder<'a, A, L>
where
    A: Clone,
    L: Clone + 'static,
{
    // pub fn add_view(&mut self, view: &LeafView<L, E>) -> &mut Self {
    //     self.views.add_view(view.with_root(self.root));
    //     self
    // }
    pub fn add_item<T: Grant<Load = PloyView<A, L>> + Backed<Back = Back>>(
        &mut self,
        item: &T,
    ) -> &mut Self {
        self.views.add_item(&item.backed(self.back));
        self
    }
    pub fn add_bare(&mut self, bare: &L) -> &mut Self {
        self.views.add_bare(bare);
        self
    }
    pub fn add_leaf(&mut self, leaf: &Sole<L>) -> &mut Self {
        self.views.add_leaf(leaf.backed(self.back));
        self
    }
    pub fn add_role(&mut self, role: &Role<A, Ploy<Sole<L>>>) -> &mut Self {
        self.views.add_role(role.backed(self.back));
        self
    }
}

impl<'a, A> AddStr for LoadViewsBuilder<'a, A, String> {
    fn add_str(&mut self, load: &str) -> &mut Self {
        self.views.push(PloyView::Bare(load.into()));
        self
    }
}
