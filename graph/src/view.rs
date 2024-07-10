use crate::*;

#[derive(Clone)]
pub enum View<I, R, E> {
    Item(I),
    Role(Role<R, E>),
}

impl<I, R, E> Solve for View<I, R, E>
where
    I: Clone + IntoRole<Load = R>,
{
    type Load = I;
    fn solve(&self) -> Self::Load {
        match self {
            View::Item(item) => item.clone(),
            View::Role(role) => I::into_role(role.solve()),
        }
    }
}

impl<I, R, E> WithRoot for View<I, R, E>
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
    // fn add_item<T: Solve<Load = Self::View>>(&mut self, item: &T);
    fn add_view(&mut self, view: &Self::View);
}

impl<I, R, E> AddToViews for Vec<View<I, R, E>>
where
    I: Clone,
    R: Clone,
    E: Clone,
{
    type View = View<I, R, E>;
    // fn add_item<T: Solve<Load = Self::View>>(&mut self, item: &T) {
    //     self.push(item.solve());
    // }
    fn add_view(&mut self, view: &Self::View) {
        self.push(view.clone());
    }
}

pub trait ToViewsBuilder<'a, I, R, E> {
    fn root(&'a mut self, reactor: &'a Root) -> ViewsBuilder<I, R, E>;
}

impl<'a, I, R, E> ToViewsBuilder<'a, I, R, E> for Vec<View<I, R, E>> {
    fn root(&'a mut self, root: &'a Root) -> ViewsBuilder<I, R, E> {
        ViewsBuilder { views: self, root }
    }
}

pub struct ViewsBuilder<'a, I, R, E> {
    views: &'a mut Vec<View<I, R, E>>,
    root: &'a Root,
}

impl<'a, I, R, E> ViewsBuilder<'a, I, R, E>
where
    I: Clone + WithRoot<Root = Root>,
    R: Clone,
    E: Clone,
{
    pub fn add_view(&mut self, view: &View<I, R, E>) -> &mut Self {
        self.views.push(view.with_root(self.root));
        self
    }
}

/////////////////////////////////////////////////

#[derive(Clone)]
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
            BaseView::Sole(leaf) => leaf.reader(read),
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
            BaseView::Bare(bare) => bare.clone().into_leaf(),
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

pub trait AddToLeafViews {
    type View;
    type Load;
    type Exact;
    fn add_item<T: Solve<Load = Self::View>>(&mut self, item: &T);
    // fn add_view(&mut self, view: Self::View);
    fn add_bare(&mut self, bare: &Self::Load);
    fn add_leaf(&mut self, leaf: Sole<Self::Load>);
    fn add_role(&mut self, role: Role<Sole<Self::Load>, Self::Exact>);
}

impl<L, E> AddToLeafViews for Vec<BaseView<L, E>>
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

pub trait ToLeafViewsBuilder<'a, L, E> {
    fn root(&'a mut self, reactor: &'a Root) -> LeafViewsBuilder<L, E>;
}

impl<'a, L, E> ToLeafViewsBuilder<'a, L, E> for Vec<BaseView<L, E>> {
    fn root(&'a mut self, root: &'a Root) -> LeafViewsBuilder<L, E> {
        LeafViewsBuilder { views: self, root }
    }
}

pub struct LeafViewsBuilder<'a, L, E> {
    views: &'a mut Vec<BaseView<L, E>>,
    root: &'a Root,
}

impl<'a, L, E> LeafViewsBuilder<'a, L, E>
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

impl<'a, E> AddStr for LeafViewsBuilder<'a, String, E> {
    fn add_str(&mut self, load: &str) -> &mut Self {
        self.views.push(BaseView::Bare(load.into()));
        self
    }
}

#[derive(Clone)]
pub enum SoleView<L> {
    Leaf(Sole<L>),
    Solver(Solver<Sole<L>>),
}

impl<L> SoleView<L> {
    pub fn new(load: L) -> Self {
        Self::Leaf(Sole::new(load))
    }
}

impl<L> Solve for SoleView<L>
where
    L: Clone,
{
    type Load = Sole<L>;
    fn solve(&self) -> Sole<L> {
        match self {
            SoleView::Leaf(leaf) => leaf.clone(),
            SoleView::Solver(solver) => solver.solve(),
        }
    }
}

// impl<I, R, E> SolveWithRoot for View<I, R, E>
// where
//     I: WithRoot<Root = Root> + IntoRole<Load = R>,
//     R: WithRoot<Root = Root>,
// {
//     type Load = I;
//     fn solve_with_root(&self, root: &Root) -> Self::Load {
//         match self {
//             View::Item(item) => item.with_root(root),
//             View::Role(role) => I::into_role(role.solve().with_root(root)),
//         }
//     }
// }
