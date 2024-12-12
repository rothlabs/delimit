use super::*;

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