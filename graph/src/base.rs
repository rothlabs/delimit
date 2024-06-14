use crate::Edge;

pub trait Base {
    fn new() -> Box<dyn Base>;
    //fn edges(&self) -> Vec<Box<dyn Base>>;
    fn compute(&self) -> Box<dyn Base>;
}

pub struct Dum {

}