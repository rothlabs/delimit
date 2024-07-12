use graph::*;

pub use sum::Sum;

mod sum;

pub type Role = TaskRole<Task, Load, Exact>;
pub type View<E> = graph::View<Item, Role, E>; 

enum Task {
    Number,
    Texels,
}
type Load = Sole<f64>;
type Item = LoadView<f64, Exact>; 
type Math<U> = Trey<U, Task, Load>;

#[derive(Clone)]
pub enum Exact {
    Sum(Math<Sum>),
}