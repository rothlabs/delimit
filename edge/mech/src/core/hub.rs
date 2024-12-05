use super::*;

mod chart;

pub struct Chart {
    pub mech: Mech,
    pub shape: Hub<Shape>,
}

impl Chart {
    pub fn grid(self, count: impl Into<Hub<u32>>) -> Hub<crate::Chart> {
        chart::Grid {
            mech: self.mech,
            shape: self.shape,
            counts: vec![count.into()]
        }
        .hub()
    }
}
