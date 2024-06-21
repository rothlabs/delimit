mod main;

pub use main::Main;

const LOAD: &str = "there should be a load";

pub trait Node {
    type U: Clone;
    type T;
    type L;
    type V;
    fn new(unit: Self::U) -> Self;
    fn unit(&self) -> &Self::U;
    fn unit_mut(&mut self) -> &mut Self::U;
    fn solve(&mut self, task: Self::T) -> Self::L;
    fn react(&mut self, vary: Self::V);
}




// pub fn flatten(&self, flat: &mut Flat) { // , state: &mut Hasher
//     let unit = serde_json::to_string(&self.read()).unwrap();
// }
// TODO: pub fn duplicate(&self) -> Node<U>  // clone and set new ID
