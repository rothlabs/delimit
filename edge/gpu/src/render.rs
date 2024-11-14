use std::ops::Range;
use super::*;

mod plan;

#[derive(Clone, Debug)]
pub struct Pass {
    stems: Vec<Hub<Mutation>>,
    entries: Vec<Entry>,   
}

#[derive(Clone, Debug)]
pub enum Entry {
    Pipe(Grc<RenderPipeline>),
    Bind(u32, Grc<BindGroup>),
    Vertex(u32, Grc<Buffer>),
    Index(Grc<Buffer>),
    Draw(Range<u32>, Range<u32>),
    DrawIndexed(Range<u32>, i32, Range<u32>),
}

#[derive(Builder, Gate, Debug)]
#[builder(pattern = "owned")]
#[builder(setter(into, strip_option))]
pub struct Plan {
    // TODO: rename roots to stems
    #[builder(default, setter(each(name = "entry", into)))]
    entries: Vec<plan::Entry>,
    #[builder(default, setter(each(name = "stem", into)))]
    stems: Vec<Hub<Mutation>>,
}

impl Plan {
    async fn map_entries(&self, post: &mut Vec<Entry>) -> graph::Result<()> {
        for cmd in &self.entries {
            match cmd {
                plan::Entry::Pipe(pipe) => post.push(Entry::Pipe(pipe.clone())),
                plan::Entry::Bind(index, bind) => {
                    let bind = bind.base().await?;
                    post
                        .push(Entry::Bind(*index, bind.clone()))
                }
                plan::Entry::Vertex(slot, buffer) => {
                    let buffer = buffer.base().await?;
                    post
                        .push(Entry::Vertex(*slot, buffer.clone()))
                }
                plan::Entry::Index(buffer) => {
                    let buffer = buffer.base().await?;
                    post.push(Entry::Index(buffer.clone()))
                    // pass.set_index_buffer(buffer.slice(..), IndexFormat::Uint16);
                }
                plan::Entry::Draw(vertices, instances) => {
                    post.push(Entry::Draw(
                        vertices.clone(),
                        instances.clone(),
                    ))
                    // pass.draw(vertices.clone(), instances.clone());
                }
                plan::Entry::DrawIndexed(indices, base_vertex, instances) => {
                    post.push(Entry::DrawIndexed(
                        indices.clone(),
                        *base_vertex,
                        instances.clone(),
                    ))
                    // pass.draw_indexed(indices.clone(), *base_vertex, instances.clone());
                }
            }
        }
        Ok(())
    }
}

impl Solve for Plan {
    type Base = Pass;
    async fn solve(&self) -> graph::Result<Hub<Pass>> {
        let mut entries = vec![];
        self.map_entries(&mut entries).await?;
        let pass = Pass {
            entries,
            stems: self.stems.clone(),
        };
        Ok(pass.into()) // Hub::Tray(Tray::Base(post))
    }
}

// TODO: turn chain.write into a trait function on Leaf<Vec<Command>>>
        // self.chain.write(|chain| chain.push(post)).await?;

impl Adapt for Plan {
    fn back(&mut self, back: &Back) -> graph::Result<()> {
        for cmd in &mut self.entries {
            if let plan::Entry::Vertex(_, buffer) = cmd {
                buffer.back(back)?
            }
        }
        self.stems.back(back)
    }
}

impl PlanBuilder {
    pub fn render(self, pipe: Grc<RenderPipeline>) -> Self {
        self.entry(plan::Entry::Pipe(pipe))
    }
    pub fn bind(self, index: u32, bind: impl Into<Hub<Grc<BindGroup>>>) -> Self {
        self.entry(plan::Entry::Bind(index, bind.into()))
    }
    pub fn vertex(self, slot: u32, buffer: impl Into<Hub<Grc<Buffer>>>) -> Self {
        self.entry(plan::Entry::Vertex(slot, buffer.into()))
    }
    pub fn index(self, buffer: impl Into<Hub<Grc<Buffer>>>) -> Self {
        self.entry(plan::Entry::Index(buffer.into()))
    }
    pub fn draw(self, vertices: Range<u32>, instances: Range<u32>) -> Self {
        self.entry(plan::Entry::Draw(vertices, instances))
    }
    pub fn draw_indexed(
        self,
        indices: Range<u32>,
        base_vertex: i32,
        instances: Range<u32>,
    ) -> Self {
        self.entry(plan::Entry::DrawIndexed(indices, base_vertex, instances))
    }
}