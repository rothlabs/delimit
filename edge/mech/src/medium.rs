use super::*;

mod pipe;

pub struct Form<'a> {
    device: &'a Device,
    layout: &'a core::pipe::Image,
    // port: &'a Viewport,
    target: &'a [Option<ColorTargetState>],
}

#[derive(Clone, Debug)]
pub struct Medium {
    pub pipe: Grc<Pipe>,
}

impl Medium {
    pub fn new(form: Form) -> Self {
        Self { pipe: Pipe::new(&form).into() }
    }
}

#[derive(Debug)]
pub struct Pipe {
    pub chart: pipe::Chart,
}

impl Pipe {
    pub fn new(form: &Form) -> Self {
        Self {
            chart: pipe::Chart::new(form),
        }
    }
}