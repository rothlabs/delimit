use crate::*;

#[derive(Clone)]
pub enum LeafView<L, E> {
    Bare(L),
    Leaf(Leaf<L>),
    Role(Role<Leaf<L>, E>),
}

impl<L, E> Default for LeafView<L, E>
where
    L: Default,
{
    fn default() -> Self {
        Self::Bare(L::default())
    }
}

impl<L, E> Reader for LeafView<L, E>
where
    L: 'static,
{
    type Unit = L;
    fn reader<F: FnOnce(&L)>(&self, read: F) {
        match self {
            LeafView::Bare(bare) => read(bare),
            LeafView::Leaf(leaf) => leaf.reader(read),
            LeafView::Role(role) => role.solve().reader(read),
        };
    }
}

// it is creating a new leaf on each solve if bare. Is this bad?
impl<L, E> Solve for LeafView<L, E>
where
    L: Clone + 'static,
{
    type Load = Leaf<L>;
    fn solve(&self) -> Leaf<L> {
        match self {
            LeafView::Bare(bare) => bare.clone().into_leaf(),
            LeafView::Leaf(leaf) => leaf.clone(),
            LeafView::Role(role) => role.solve(),
        }
    }
}

impl<L, E> WithRoot for LeafView<L, E>
where
    L: Clone,
    E: Clone,
{
    type Root = Reactor;
    fn with_root(&self, root: &Self::Root) -> Self {
        match self {
            LeafView::Bare(bare) => LeafView::Bare(bare.clone()),
            LeafView::Leaf(leaf) => LeafView::Leaf(leaf.with_root(root)),
            LeafView::Role(role) => LeafView::Role(role.with_root(root)),
        }
    }
}

pub trait AddToLeafViews<L, E> {
    type Load;
    type Exact;
    fn add(&mut self, item: &LeafView<L, E>) -> &mut Self;
    fn add_bare(&mut self, bare: &Self::Load) -> &mut Self;
    fn add_leaf(&mut self, leaf: &Leaf<Self::Load>, reactor: &Reactor);
    fn add_role(
        &mut self,
        role: &Role<Leaf<Self::Load>, Self::Exact>,
        reactor: &Reactor,
    ) -> &mut Self;
}

impl<L, E> AddToLeafViews<L, E> for Vec<LeafView<L, E>>
where
    L: Clone,
    E: Clone,
{
    type Load = L;
    type Exact = E;
    fn add(&mut self, item: &LeafView<L, E>) -> &mut Self {
        self.push(item.clone());
        self
    }
    fn add_bare(&mut self, bare: &L) -> &mut Self {
        self.push(LeafView::Bare(bare.clone()));
        self
    }
    fn add_leaf(&mut self, leaf: &Leaf<L>, reactor: &Reactor) {
        self.push(LeafView::Leaf(leaf.with_root(reactor)));
    }
    fn add_role(&mut self, role: &Role<Leaf<L>, E>, reactor: &Reactor) -> &mut Self {
        self.push(LeafView::Role(role.with_root(reactor)));
        self
    }
}

pub trait AddStr {
    fn add_str(&mut self, load: &str) -> &mut Self;
}

impl<E> AddStr for Vec<LeafView<String, E>> {
    fn add_str(&mut self, load: &str) -> &mut Self {
        self.push(LeafView::Bare(load.into()));
        self
    }
}

pub trait ToLeafViewsBuilder<'a, L, E> {
    fn reactor(&'a mut self, reactor: &'a Reactor) -> LeafViewsBuilder<L, E>;
}

impl<'a, L, E> ToLeafViewsBuilder<'a, L, E> for Vec<LeafView<L, E>> {
    fn reactor(&'a mut self, reactor: &'a Reactor) -> LeafViewsBuilder<L, E> {
        LeafViewsBuilder {
            views: self,
            reactor,
        }
    }
}

pub struct LeafViewsBuilder<'a, L, E> {
    views: &'a mut Vec<LeafView<L, E>>,
    reactor: &'a Reactor,
}

impl<'a, L, E> LeafViewsBuilder<'a, L, E>
where
    L: Clone,
    E: Clone,
{
    pub fn add<T: SolveWithReactor<Item = LeafView<L, E>>>(&mut self, item: &T) -> &mut Self {
        self.views.push(item.solve_with_reactor(&self.reactor));
        self
    }
    pub fn add_bare(&mut self, bare: &L) -> &mut Self {
        self.views.add_bare(bare);
        self
    }
    pub fn add_leaf(&mut self, leaf: &Leaf<L>) -> &mut Self {
        self.views.add_leaf(leaf, self.reactor);
        self
    }
    pub fn add_role(&mut self, role: &Role<Leaf<L>, E>) -> &mut Self {
        self.views.add_role(role, self.reactor);
        self
    }
}

impl<'a, E> AddStr for LeafViewsBuilder<'a, String, E> {
    fn add_str(&mut self, load: &str) -> &mut Self {
        self.views.push(LeafView::Bare(load.into()));
        self
    }
}

#[derive(Clone)]
pub enum LeafEye<L> {
    Leaf(Leaf<L>),
    Solver(Solver<Leaf<L>>),
}

impl<L> LeafEye<L> {
    pub fn new(load: L) -> Self {
        Self::Leaf(Leaf::new(load))
    }
}

impl<L> Solve for LeafEye<L>
where
    L: Clone,
{
    type Load = Leaf<L>;
    fn solve(&self) -> Leaf<L> {
        match self {
            LeafEye::Leaf(leaf) => leaf.clone(),
            LeafEye::Solver(solver) => solver.solve(),
        }
    }
}