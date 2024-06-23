pub trait New {
    type Unit;
    fn new(unit: Self::Unit) -> Self;
}
