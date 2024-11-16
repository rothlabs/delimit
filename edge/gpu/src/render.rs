use super::*;
use std::ops::Range;

mod codec;
pub mod pass;

#[derive(Clone, Debug)]
pub struct Pass {
    pub steps: Vec<Step>,
}

#[derive(Clone, Debug)]
pub enum Step {
    Pipe(Grc<RenderPipeline>),
    Bind(u32, Grc<BindGroup>),
    Vertex(u32, Grc<Buffer>),
    Index(Grc<Buffer>),
    Draw(Range<u32>, Range<u32>),
    DrawIndexed(Range<u32>, i32, Range<u32>),
}

#[derive(Builder, BuildGate, Debug)]
#[builder(pattern = "owned")]
#[builder(setter(into, strip_option))]
pub struct Codec {
    #[builder(default, setter(each(name = "step", into)))]
    steps: Vec<codec::Step>,
    #[builder(default, setter(each(name = "stem", into)))]
    stems: Vec<Hub<Mutation>>,
}

impl Solve for Codec {
    type Base = Vec<Step>;
    async fn solve(&self) -> node::Result<Vec<Step>> {
        // TODO: put stems in gpu::BindGroup
        self.stems.depend().await?;
        let mut entries = vec![];
        for cmd in &self.steps {
            match cmd {
                codec::Step::Pipe(pipe) => entries.push(Step::Pipe(pipe.clone())),
                codec::Step::Bind(index, bind) => {
                    let bind = bind.base().await?;
                    entries.push(Step::Bind(*index, bind.clone()))
                }
                codec::Step::Vertex(slot, buffer) => {
                    let buffer = buffer.base().await?;
                    entries.push(Step::Vertex(*slot, buffer.clone()))
                }
                codec::Step::Index(buffer) => {
                    let buffer = buffer.base().await?;
                    entries.push(Step::Index(buffer.clone()))
                }
                codec::Step::Draw(vertices, instances) => {
                    entries.push(Step::Draw(vertices.clone(), instances.clone()))
                }
                codec::Step::DrawIndexed(indices, base_vertex, instances) => entries.push(
                    Step::DrawIndexed(indices.clone(), *base_vertex, instances.clone()),
                ),
            }
        }
        Ok(entries.into_leaf().into())
    }
}

impl Adapt for Codec {
    fn back(&mut self, back: &Back) -> graph::Result<()> {
        for cmd in &mut self.steps {
            if let codec::Step::Vertex(_, buffer) = cmd {
                buffer.back(back)?
            }
        }
        self.stems.back(back)
    }
}

impl CodecBuilder {
    pub fn pipe(self, pipe: Grc<RenderPipeline>) -> Self {
        self.step(codec::Step::Pipe(pipe))
    }
    pub fn bind(self, index: u32, bind: impl Into<Hub<Grc<BindGroup>>>) -> Self {
        self.step(codec::Step::Bind(index, bind.into()))
    }
    pub fn vertex(self, slot: u32, buffer: impl Into<Hub<Grc<Buffer>>>) -> Self {
        self.step(codec::Step::Vertex(slot, buffer.into()))
    }
    pub fn index(self, buffer: impl Into<Hub<Grc<Buffer>>>) -> Self {
        self.step(codec::Step::Index(buffer.into()))
    }
    pub fn draw(self, vertices: Range<u32>, instances: Range<u32>) -> Self {
        self.step(codec::Step::Draw(vertices, instances))
    }
    pub fn draw_indexed(
        self,
        indices: Range<u32>,
        base_vertex: i32,
        instances: Range<u32>,
    ) -> Self {
        self.step(codec::Step::DrawIndexed(indices, base_vertex, instances))
    }
}

// TODO: turn chain.write into a trait function on Leaf<Vec<Command>>>
// self.chain.write(|chain| chain.push(post)).await?;
