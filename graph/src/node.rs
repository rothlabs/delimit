mod leaf;
mod solver;
mod reactor;
mod responder;

pub use leaf::Leaf;
pub use solver::Solver;
pub use reactor::Reactor;
pub use responder::Responder;

pub trait New {
    type Unit;
    fn new(unit: Self::Unit) -> Self;
}

pub trait Read {
    type Unit;
    fn read(&self) -> &Self::Unit;
}

pub trait Write {
    type Unit;
    fn write(&mut self) -> &mut Self::Unit;
}

// TODO: move to basis.rs because it can be used for units as well?
pub trait Solve { 
    type Task;
    type Load;
    fn solve(&mut self, task: Self::Task) -> Self::Load;
}

pub trait React {
    fn react(&mut self);
}

pub trait Respond {
    type Memo;
    fn respond(&mut self, memo: Self::Memo);
}



// pub fn flatten(&self, flat: &mut Flat) { // , state: &mut Hasher
//     let unit = serde_json::to_string(&self.read()).unwrap();
// }
// TODO: pub fn duplicate(&self) -> Node<U>  // clone and set new ID
