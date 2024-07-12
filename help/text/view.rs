use crate::plain::*;

pub type View<E> = graph::View<Item, Role, E>;

// #[derive(Clone)]
// pub enum View<E> {
//     Text(Item),
//     Role(graph::Role<Role, E>),
// }

// impl<E> SolveWithRoot for View<E> {
//     type Item = Item;
//     fn solve_with_root(&self, root: &Root) -> Self::Item {
//         match self {
//             View::Text(item) => item.with_root(root),
//             View::Role(role) => Item::Role(role.solve().with_root(root)),
//         }
//     }
// }

// impl<E> View<E>
// where
//     E: Clone,
// {
//     pub fn root(&self, root: &Root) -> Self {
//         match self {
//             View::Text(text) => View::Text(text.with_root(root)),
//             View::Role(role) => View::Role(role.with_root(root)),
//         }
//     }
// }

// pub trait ToPlain {
//     fn plain(&self, reactor: &Reactor) -> ;
// }

// impl<E> ToPlain for graph::Role<Role, E> {
//     fn plain(&self, reactor: &Reactor) {

//     }
// }

// pub trait AddToViews<L, E> {
//     type Load;
//     type Exact;
//     // fn add(&mut self, item: View<E>) -> &mut Self;
//     // fn add_bare(&mut self, bare: &Self::Load) -> &mut Self;
//     // fn add_leaf(&mut self, leaf: &Leaf<Self::Load>, reactor: &Reactor);
//     fn add_role(
//         &mut self,
//         role: &graph::Role<Role, Self::Exact>,
//         reactor: &Reactor,
//     ) -> &mut Self;
// }

// impl<E> AddToViews for Vec< {

// }