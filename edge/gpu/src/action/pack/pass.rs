use super::*;

#[derive(Debug)]
pub struct Compute {
    pub pipe: Grc<ComputePipeline>,
    pub binds: Vec<GroupBind>,
    pub kind: compute::Kind,
}

pub mod compute {
    use super::*;
    #[derive(Debug)]
    pub enum Kind {
        Dispatch(u32),
        Indirect,
    }
}

#[derive(Debug)]
pub struct Render {
    pub pipe: Grc<RenderPipeline>,
    pub groups: Vec<GroupBind>,
    pub buffers: Vec<BufferBind>,
    pub kind: render::Kind,
}

pub mod render {
    use super::*;
    #[derive(Debug)]
    pub enum Kind {
        Draw(Draw),
        Other,
    }
}
