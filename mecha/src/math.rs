use graph::*;

pub use add::Sum;

mod add;

pub type Role = graph::SolveRole<Load, Exact>;
pub type View<E> = graph::View<Item, Role, E>; 

type Load = Sole<f64>;
type Item = BaseView<f64, Exact>; 
type Math<U> = Pair<U, Load>;

#[derive(Clone)]
pub enum Exact {
    Sum(Math<Sum>),
}