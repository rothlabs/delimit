use super::*;
use std::ops::Range;

#[derive(Default, Clone, Debug)]
pub struct Command {
    // TODO: put render pass descriptor here and remove msaa
    pub msaa: u32,
    pub compute: Vec<compute::Entry>,
    pub render: Vec<Render>,
}

pub mod compute {
    use super::*;
    #[derive(Clone, Debug)]
    pub enum Entry {
        Pipe(Grc<ComputePipeline>),
        Bind(u32, Grc<BindGroup>),
        Dispatch(u32),
    }
}

// pub mod render {
    // use super::*;
    #[derive(Clone, Debug)]
    pub enum Render {
        Pipe(Grc<RenderPipeline>),
        Bind(u32, Grc<BindGroup>),
        Vertex(u32, Grc<Buffer>),
        Index(Grc<Buffer>),
        Draw(Range<u32>, Range<u32>),
        DrawIndexed(Range<u32>, i32, Range<u32>),
    }
// }

// pub struct RenderPlan
