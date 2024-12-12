use super::*;

pub mod form;
pub mod hedge;

/// Specify how to create `Weft` evaluations.
#[derive(Clone, Debug, Default)]
pub struct Form {
    pub travel: Option<form::Travel>,
    pub orient: Option<form::Orient>,
    pub spline: BTreeMap<u32, form::Spline>,
}

// TODO: Flow should be in an enum of other kinds of operations like ButtJoin
/// Index into Warp and `Weft`.
#[derive(Clone, Debug, Default)]
pub struct Flow {
    pub travel: Option<Hedge>,
    pub orient: Option<Hedge>,
    pub spline: BTreeMap<u32, Hedge>,
}
