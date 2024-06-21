mod node;
mod leaf;
mod reactor;

pub use node::Node;
pub use leaf::Leaf;
pub use reactor::Reactor;

pub trait Read {
    type Unit;
    fn read(&self) -> &Self::Unit;
}

pub trait Write {
    type Unit;
    fn write(&mut self) -> &mut Self::Unit;
}

pub trait Solve {
    type Task;
    type Load;
    fn solve(&mut self, task: Self::Task) -> Self::Load;
}

pub trait React {
    type Vary;
    fn react(&mut self, vary: Self::Vary);
}

// pub fn flatten(&self, flat: &mut Flat) { // , state: &mut Hasher
//     let unit = serde_json::to_string(&self.read()).unwrap();
// }
// TODO: pub fn duplicate(&self) -> Node<U>  // clone and set new ID
