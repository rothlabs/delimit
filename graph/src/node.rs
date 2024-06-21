pub mod main;
pub mod reactive;

pub use main::Node;
//pub use reactive;

const LOAD: &str = "there should be a load";

pub trait UnitRef {
    type Unit: Clone;
    fn unit(&self) -> &Self::Unit;
    fn unit_mut(&mut self) -> &mut Self::Unit;
}

pub trait Solve {
    type Task;
    type Load; //: Clone;
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
