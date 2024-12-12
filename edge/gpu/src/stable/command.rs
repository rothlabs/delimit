use super::*;

#[derive(Debug)]
pub enum Kind {
    Leaf,
    Dispatch(Dispatch),
    Draw(Draw),
    // CopyBuffer,
}


#[derive(Debug)]
pub struct Dispatch {
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
pub struct Draw {
    pub pipe: Grc<RenderPipeline>,
    pub groups: Vec<stable::GroupBind>,
    pub buffers: Vec<stable::BufferBind>,
    pub kind: draw::Kind,
}

pub mod draw {
    use super::*;
    #[derive(Debug)]
    pub enum Kind {
        Direct(flat::command::draw::Direct),
        Indirect,
    }
}
