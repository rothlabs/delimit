use crate::*;

#[derive(Clone, Serialize)]
pub enum TaskView<L, E> {
    Bare(L),
    Sole(Sole<L>),
    Role(SolveRole<Sole<L>, E>),
}

impl<L, E> IntoRole for TaskView<L, E> {
    type Load = SolveRole<Sole<L>, E>;
    fn into_role(load: Self::Load) -> Self {
        Self::Role(load)
    }
}

impl<L, E> Default for TaskView<L, E>
where
    L: Default,
{
    fn default() -> Self {
        Self::Bare(L::default())
    }
}

impl<L, E> Reader for TaskView<L, E>
where
    L: 'static + Send + Sync,
{
    type Unit = L;
    fn reader<F: FnOnce(&L)>(&self, read: F) {
        match self {
            TaskView::Bare(bare) => read(bare),
            TaskView::Sole(sole) => sole.reader(read),
            TaskView::Role(role) => role.grant().reader(read),
        };
    }
}

// it is creating a new leaf on each solve if bare. Is this bad?
impl<L, E> Grant for TaskView<L, E>
where
    L: Clone + 'static,
{
    type Load = Sole<L>;
    fn grant(&self) -> Sole<L> {
        match self {
            TaskView::Bare(bare) => bare.clone().into_sole(),
            TaskView::Sole(leaf) => leaf.clone(),
            TaskView::Role(role) => role.grant(),
        }
    }
}

impl<L, E> WithRoot for TaskView<L, E>
where
    L: Clone,
    E: Clone,
{
    type Root = Root;
    fn with_root(&self, root: &Self::Root) -> Self {
        match self {
            TaskView::Bare(bare) => TaskView::Bare(bare.clone()),
            TaskView::Sole(leaf) => TaskView::Sole(leaf.with_root(root)),
            TaskView::Role(role) => TaskView::Role(role.with_root(root)),
        }
    }
}

pub trait AddToTaskViews {
    type View;
    type Load;
    type Exact;
    fn add_item<T: Grant<Load = Self::View>>(&mut self, item: &T);
    // fn add_view(&mut self, view: Self::View);
    fn add_bare(&mut self, bare: &Self::Load);
    fn add_leaf(&mut self, leaf: Sole<Self::Load>);
    fn add_role(&mut self, role: SolveRole<Sole<Self::Load>, Self::Exact>);
}

impl<L, E> AddToTaskViews for Vec<TaskView<L, E>>
where
    L: Clone + 'static,
    E: Clone,
{
    type View = TaskView<L, E>;
    type Load = L;
    type Exact = E;
    fn add_item<T: Grant<Load = Self::View>>(&mut self, item: &T) {
        self.push(item.grant());
    }
    // fn add_view(&mut self, item: Self::View) {
    //     self.push(item);
    // }
    fn add_bare(&mut self, bare: &L) {
        self.push(TaskView::Bare(bare.clone()));
    }
    fn add_leaf(&mut self, leaf: Sole<L>) {
        self.push(TaskView::Sole(leaf));
    }
    fn add_role(&mut self, role: SolveRole<Sole<L>, E>) {
        self.push(TaskView::Role(role));
    }
}

pub trait ToTaskViewsBuilder<'a, L, E> {
    fn root(&'a mut self, reactor: &'a Root) -> TaskViewsBuilder<L, E>;
}

impl<'a, L, E> ToTaskViewsBuilder<'a, L, E> for Vec<TaskView<L, E>> {
    fn root(&'a mut self, root: &'a Root) -> TaskViewsBuilder<L, E> {
        TaskViewsBuilder { views: self, root }
    }
}

pub struct TaskViewsBuilder<'a, L, E> {
    views: &'a mut Vec<TaskView<L, E>>,
    root: &'a Root,
}

impl<'a, L, E> TaskViewsBuilder<'a, L, E>
where
    L: Clone + 'static,
    E: Clone,
{
    pub fn add_item<T: Grant<Load = TaskView<L, E>> + WithRoot<Root = Root>>(
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
    pub fn add_role(&mut self, role: &SolveRole<Sole<L>, E>) -> &mut Self {
        self.views.add_role(role.with_root(self.root));
        self
    }
}

impl<'a, E> AddStr for TaskViewsBuilder<'a, String, E> {
    fn add_str(&mut self, load: &str) -> &mut Self {
        self.views.push(TaskView::Bare(load.into()));
        self
    }
}
