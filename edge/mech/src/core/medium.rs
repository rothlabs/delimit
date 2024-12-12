use super::*;

pub mod image;

mod pipe;

pub struct Form<'a> {
    pub mech: Mech,
    pub device: &'a Device,
    pub layout: &'a core::pipe::Image,
    pub target: &'a [Option<ColorTargetState>],
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

pub struct Image<'a> {
    pub index: Hub<u32>,
    pub bind: Hub<gpu::stable::GroupBind>,
    pub medium: &'a Medium,
}

// pub struct Image<'a> {
//     pipe: Hub<Grc<RenderPipeline>>,
//     bind: &'a core::group::bind::Bank,
// }

// pub fn image(&self, pipe: impl Into<Hub<Grc<RenderPipeline>>>) -> Image {
//     Image {
//         pipe: pipe.into(),
//         bind: &self.mech.group.bind,
//     }
// }
