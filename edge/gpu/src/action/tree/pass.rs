use super::*;

#[derive(Debug)]
pub struct Compute {
    pub pipe: Grc<ComputePipeline>,
    pub binds: Vec<stable::GroupBind>,
    pub kind: dispatch::Kind,
}

pub mod dispatch {
    use super::*;
    #[derive(Debug)]
    pub enum Kind {
        Direct(u32),
        Indirect,
    }
}

#[derive(Debug)]
pub struct Render {
    pub pipe: Grc<RenderPipeline>,
    pub groups: Vec<stable::GroupBind>,
    pub buffers: Vec<stable::BufferBind>,
    pub kind: render::Kind,
}

pub mod render {
    use super::*;
    #[derive(Debug)]
    pub enum Kind {
        Direct(stable::command::draw::Direct),
        Indirect,
    }
}
