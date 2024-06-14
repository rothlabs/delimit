pub trait Compute<P> {
    //fn edges(&self) -> Vec<Box<dyn Base>>;
    fn compute(&self) -> Option<P>;
}

impl<P> Compute<P> for String {
    fn compute(&self) -> Option<P> {
        None
    }
}
