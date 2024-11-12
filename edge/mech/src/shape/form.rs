use super::*;

#[derive(Builder, Clone, Default, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "graph::Error"))]
#[builder(setter(strip_option))]
pub struct Travel {
    pub extrude: Option<Hedge>,
    // per component function
    // https://www.youtube.com/watch?v=AjDU7eegt4g
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
    pub basis: Option<Hedge>,
    pub nurbs: Option<Hedge>,
}
