use super::*;

#[derive(Debug)]
pub enum Step {
    Pipe(Grc<RenderPipeline>),
    Bind(u32, Hub<Grc<BindGroup>>),
    // TODO: take Hedge so hedge.root.depend is automatic
    Vertex(u32, Hub<Grc<Buffer>>),
    Index(Hub<Grc<Buffer>>),
    // TODO: these ranges need to be Hubs!!!!!!!!!!!!!!!!!!!!!!!!!
    Draw(Range<u32>, Range<u32>),
    DrawIndexed(Range<u32>, i32, Range<u32>),
}
