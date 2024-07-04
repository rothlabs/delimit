use crate::*;

pub enum View<L: Reader, E> {
    Bare(L::Unit),
    Leaf(Leaf<L::Unit>),
    Role(Role<L, E>),
}

impl<L, E> Reader for View<L, E> 
where 
    L: Reader + 'static, 
{
    type Unit = L::Unit;
    fn reader<F: FnOnce(&L::Unit)>(&self, read: F) {
        match self {
            View::Bare(bare) => read(bare),
            View::Leaf(leaf) => leaf.reader(read),
            View::Role(role) => role.solver.solve().reader(read),
        };
    }
}

pub trait AddToView {
    type Load: Reader;
    type Exact;
    fn add_bare(&mut self, bare: &<Self::Load as Reader>::Unit) -> &mut Self;
    fn add_leaf(&mut self, leaf: &Leaf<<Self::Load as Reader>::Unit>, reactor: &Reactor);
    fn add_role(&mut self, role: &Role<Self::Load, Self::Exact>, reactor: &Reactor);
}

impl<L, E> AddToView for Vec<View<L, E>> 
where 
    L: Reader + Clone,
    L::Unit: Clone,
    E: Clone,
{
    type Load = L;
    type Exact = E;
    fn add_bare(&mut self, bare: &L::Unit) -> &mut Self {
        self.push(View::Bare(bare.clone()));
        self
    }
    fn add_leaf(&mut self, leaf: &Leaf<L::Unit>, reactor: &Reactor) {
        self.push(View::Leaf(leaf.with_reactor(reactor)));
    }
    fn add_role(&mut self, role: &Role<L, E>, reactor: &Reactor) {
        self.push(View::Role(role.with_reactor(reactor)));
    }
}

pub trait AddStr {
    fn add_str(&mut self, load: &str) -> &mut Self;
}

impl<E> AddStr for Vec<View<Leaf<String>, E>> {
    fn add_str(&mut self, load: &str) -> &mut Self {
        self.push(View::Bare(load.to_owned()));
        self
    }
}

#[derive(Clone)]
pub struct Role<L, E> {
    pub exact: E,
    pub solver: Solver<L>,
}

impl<L, E> WithReactor for Role<L, E> 
where 
    E: Clone,
{
    fn with_reactor(&self, reactor: &Reactor) -> Self {
        Self {
            exact: self.exact.clone(),
            solver: self.solver.with_reactor(reactor),
        }
    }
}

// pub trait ToViewsBuilder<L: Reader, E> {
//     fn builder(&mut self) -> ViewsBuilder<L, E>;
// }

// impl<L: Reader, E> ToViewsBuilder<L, E> for Vec<View<L, E>> {
//     fn builder(&mut self) -> ViewsBuilder<L, E> {
//         ViewsBuilder { views: self, reactor: None }
//     }
// }

// pub struct ViewsBuilder<'a, L: Reader, E> {
//     views: &'a mut Vec<View<L, E>>,
//     reactor: Option<&'a Reactor>, 
// }

// impl<'a, L: Reader, E> ViewsBuilder<'a, L, E> {
    
// }





// pub trait AddLeaf {
//     type Item;
//     fn add_leaf(&mut self, bare: &Self::Item);
// }

// impl<L, E> AddLeaf for Vec<View<L, E>> 
// where 
//     L: Reader + Clone,
// {
//     type Item = L::Unit;
//     fn add_leaf(&mut self, leaf: &Leaf<L>) {
//         self.push(View::Leaf(leaf.clone()));
//     }
// }


// impl<L, E> Solve for Role<L, E> {
//     type Load = L;
//     fn solve(&self) -> Self::Load {
//         self.solver.solve()
//     }
// }


// impl<L, E> Reader for View<L, E> 
// where 
//     L: Reader + 'static, 
// {
//     type Unit = L::Unit;
//     fn reader<F: FnOnce(&Self::Unit)>(&self, read: F) {
//         self.solver.solve().reader(read);
//     }
// }

// impl<L, E> Solve for Viewer<L, E> 
// where 
//     L: Clone,
// {
//     type Load = L;
//     fn solve(&self) -> L {
//         match self {
//             Viewer::Bare(unit) => unit.clone(),
//             Viewer::Leaf(leaf) => leaf.solve(),
//             Viewer::View(view) => view.solver.solve(),
//         }
//     }
// }