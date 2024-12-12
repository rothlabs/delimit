use super::*;

mod chart;

pub struct Chart<'a> {
    pub mech: &'a Mech,
    pub shape: Hub<flat::Shape>,
}

impl Chart<'_> {
    pub fn grid(self, count: impl Into<Hub<u32>>) -> Hub<crate::Chart> {
        chart::Grid {
            mech: self.mech.clone(),
            shape: self.shape,
            counts: vec![count.into()],
        }
        .hub()
    }
}
