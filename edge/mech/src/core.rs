use super::*;
use store::*;

pub mod group;
pub mod pipe;
pub mod medium;

mod store;
mod mech;

#[derive(Clone, Debug)]
pub struct Mech {
    pub gpu: Gpu,
    pub pipe: Grc<mech::Pipe>,
    pub group: group::Bank,
    pub store: Store,
}

// TODO: Mech should be made from Gpu and mech::View should be made from gpu::Viewport
// mech can still contain all the Pipe info
impl Mech {
    pub fn new(gpu: &Gpu) -> Self {
        let store = Store::new(gpu);
        let layout = group::layout::Bank::new(&gpu.device, &store);
        let group = group::Bank::new(&layout);
        Self {
            gpu: gpu.clone(),
            pipe: mech::Pipe::new(&layout).into(),
            group,
            store,
        }
    }
    pub fn chart(&self, shape: impl Into<Hub<flat::Shape>>) -> mech::Chart {
        mech::Chart {
            mech: self,
            shape: shape.into(),
        }
    }
    pub fn medium(&self, target: &[Option<ColorTargetState>]) -> Medium {
        let form = medium::Form {
            mech: self.clone(),
            device: &self.gpu.device,
            layout: &self.pipe.image,
            target,
        };
        Medium::new(form)
    }
    pub fn rig(&self) -> (Hub<u32>, Hub<gpu::stable::GroupBind>) {
        let index = self.store.rig(1);
        let bind = gpu::active::group::Bind {
            slot: 1.into(),
            group: self.group.rig.clone(),
            offsets: vec![index.clone()],
        }
        .hub();
        (index, bind)
    }
}

#[derive(Clone, Debug)]
pub struct Medium {
    pub mech: Mech,
    pub pipe: Grc<medium::Pipe>,
}

impl Medium {
    pub fn new(form: medium::Form) -> Self {
        Self {
            pipe: medium::Pipe::new(&form).into(),
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
    pub fn image(&self) -> medium::Image {
        let (index, bind) = self.mech.rig();
        medium::Image {
            index,
            bind,
            medium: self,
        }
    }
}


// pub fn travel(&self) -> TravelBuilder {
//     TravelBuilder::default()
// }
// pub fn orient(&self) -> OrientBuilder {
//     OrientBuilder::default()
// }
// pub fn spline(&self) -> SplineBuilder {
//     SplineBuilder::default()
// }
// pub fn form(&self) -> FormBuilder {
//     FormBuilder::default()
// }
// pub fn flow(&self) -> FlowBuilder {
//     FlowBuilder::default()
// }
// Shape builder with dimensionality.

// // TODO: also impl on shape to creating everything needed to render automatically
// pub fn draw(&self, plot: impl Into<Hub<Plot>>) -> make::Draw {
//     make::Draw {
//         core: self.clone(),
//         plot: plot.into(),
//     }
// }

// #[derive(Debug)]
// pub struct RigBind {
//     pub bind: Hub<stable::GroupBind>,
//     pub index: Hub<u32>,
// }
