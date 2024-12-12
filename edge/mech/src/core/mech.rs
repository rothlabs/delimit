use super::*;

mod chart;

#[derive(Debug)]
pub struct Pipe {
    pub chart: pipe::Chart,
    pub image: pipe::Image,
}

impl Pipe {
    pub fn new(layout: &group::layout::Bank) -> Self {
        Self {
            chart: pipe::Chart::new(layout),
            image: pipe::Image::new(layout),
        }
    }
}

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