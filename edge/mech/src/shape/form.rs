use super::*;

#[derive(Builder, Clone, Default, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "graph::Error"))]
#[builder(setter(strip_option))]
pub struct Travel {
    pub extrude: Option<Hedge>,
}

#[derive(Builder, Clone, Default, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "graph::Error"))]
#[builder(setter(strip_option))]
pub struct Orient {
    pub revolve: Option<Hedge>,
}

#[derive(Builder, Clone, Default, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "graph::Error"))]
#[builder(setter(strip_option))]
pub struct Spline {
    // alt name for basis: knots
    pub basis: Option<Hedge>,
    pub nurbs: Option<Hedge>,
}
