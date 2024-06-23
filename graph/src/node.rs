mod leaf;
mod reactor;
mod responder;
mod solver;

pub use leaf::Leaf;
pub use reactor::Reactor;
pub use responder::Responder;
pub use solver::Solver;

pub trait Read {
    type Unit;
    fn read(&self) -> &Self::Unit;
}

pub trait Write {
    type Unit;
    fn write<F: FnOnce(&mut Self::Unit)>(&mut self, write: F); //  -> &mut Self::Unit
}

// TODO: move to base.rs because it can be used for units as well?
// pub trait Solve {
//     type Task;
//     type Load;
//     fn solve(&mut self, task: Self::Task) -> Self::Load;
// }

pub trait React {
    fn react(&mut self);
}

pub trait Respond {
    type Memo;
    fn respond(&mut self, memo: Self::Memo);
}

// pub trait AddLink {
//     type Unit;
//     type Link;
//     fn add_link<F: FnOnce(&mut Self::Unit, Self::Link)>(&mut self, link: Self::Link, add: F);
// }

// pub fn flatten(&self, flat: &mut Flat) { // , state: &mut Hasher
//     let unit = serde_json::to_string(&self.read()).unwrap();
// }
// TODO: pub fn duplicate(&self) -> Node<U>  // clone and set new ID
