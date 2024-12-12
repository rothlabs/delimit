use super::*;

pub mod form;

mod chart;

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

pub struct Chart<'a> {
    pub mech: &'a Mech,
    pub shape: &'a Shape,
}

impl<'a> Chart<'a> {
    pub fn grid(&self, counts: &'a [Hub<u32>]) -> Result<Hedge> {
        chart::Grid {
            mech: self.mech,
            shape: self.shape,
            counts,
        }
        .hedge()
    }
}
