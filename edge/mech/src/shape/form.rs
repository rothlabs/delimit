use super::*;

#[derive(Clone, Default, Debug)]
pub struct Travel {
    pub extrude: Option<Hedge>,
}

#[derive(Clone, Default, Debug)]
pub struct Orient {
    pub revolve: Option<Hedge>,
}

#[derive(Clone, Default, Debug)]
pub struct Spline {
    // alt name: knots (because hedge only contains knots)
    pub basis: Option<Hedge>,
    pub nurbs: Option<Hedge>,
}
