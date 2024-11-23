use super::*;

#[derive(Debug)]
pub struct Pass {
    pub binds: Vec<Bind>,
    pub kind: Kind,
}

#[derive(Debug)]
pub enum Kind {
    Compute(Compute),
    Render(Render),
}

#[derive(Debug)]
pub struct Compute {
    pub pipe: Grc<ComputePipeline>,
    pub kind: compute::Kind,
}

pub mod compute {
    use super::*;
    #[derive(Debug)]
    pub enum Kind {
        Dispatch(Dispatch),
        Indirect,
    }
    #[derive(Debug)]
    pub struct Dispatch {
        pub size: u32,
    }
}

#[derive(Debug)]
pub struct Render {
    pub pipe: Grc<RenderPipeline>,
    pub vertex: Vertex,
    pub kind: render::Kind,
}

pub mod render {
    use super::*;
    #[derive(Debug)]
    pub enum Kind {
        Draw(Draw),
        Other,
    }
    #[derive(Debug)]
    pub struct Draw {
        pub vertices: Range<u32>,
        pub instances: Range<u32>,
    }
}
