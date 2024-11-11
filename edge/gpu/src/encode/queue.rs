use std::ops::Range;
use super::*;

pub mod compute {
    use super::*;
    pub enum Entry {
        Pipe(Grc<ComputePipeline>),
        Bind(u32, Hub<Grc<BindGroup>>),
        Dispatch(Hub<u32>),
    }
}

pub mod render {
    use super::*;
    pub enum Entry {
        Pipe(Grc<RenderPipeline>),
        Bind(u32, Grc<BindGroup>),
        Vertex(u32, Grc<Buffer>),
        Index(Grc<Buffer>),
        Draw(Range<u32>, Range<u32>),
        DrawIndexed(Range<u32>, i32, Range<u32>),
    }
}