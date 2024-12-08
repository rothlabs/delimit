use super::*;

pub mod image;

mod pipe;

pub struct Form<'a> {
    pub mech: Mech,
    pub device: &'a Device,
    pub layout: &'a core::pipe::Image,
    pub target: &'a [Option<ColorTargetState>],
}

#[derive(Clone, Debug)]
pub struct Medium {
    pub mech: Mech,
    pub pipe: Grc<Pipe>,
}

impl Medium {
    pub fn new(form: Form) -> Self {
        Self {
            pipe: Pipe::new(&form).into(),
            mech: form.mech,
        }
    }
    pub fn view(&self, size: Leaf<(u32, u32)>) -> View {
        View {
            gpu: self.mech.gpu.clone(),
            mech: self.mech.clone(),
            medium: self.clone(),
            size,
        }
    }
    pub fn image(&self) -> Image {
        let (index, bind) = self.mech.rig();
        Image {
            index,
            bind,
            medium: self,
        }
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

pub struct Image<'a> {
    pub index: Hub<u32>,
    bind: Hub<gpu::stable::GroupBind>,
    medium: &'a Medium,
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
