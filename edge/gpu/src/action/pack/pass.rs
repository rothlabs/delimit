use super::*;

// #[derive(Debug)]
// pub enum Kind {
//     Compute(Compute),
//     Render(Render),
// }

#[derive(Debug)]
pub struct Compute {
    pub pipe: Grc<ComputePipeline>,
    pub binds: Vec<Bind>,
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
    pub binds: Vec<Bind>,
    pub buffers: Vec<Vertex>,
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
