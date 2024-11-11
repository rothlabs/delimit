use std::ops::Range;
use super::*;

#[derive(Default, Clone, Debug)]
pub struct Command {
    pub msaa: bool,
    pub compute: Vec<compute::Entry>,
    pub render: Vec<render::Entry>,
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

pub mod render {
    use super::*;
    #[derive(Clone, Debug)]
    pub enum Entry {
        Pipe(Grc<RenderPipeline>),
        Bind(u32, Grc<BindGroup>),
        Vertex(u32, Grc<Buffer>),
        Index(Grc<Buffer>),
        Draw(Range<u32>, Range<u32>),
        DrawIndexed(Range<u32>, i32, Range<u32>),
    }
}