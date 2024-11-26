use super::*;
use std::ops::Range;

#[derive(Builder, Gate, Debug)]
#[builder(pattern = "owned")]
#[builder(setter(into, strip_option))]
pub struct Render {
    // TODO: rename roots to stems
    #[builder(default, setter(each(name = "root", into)))]
    roots: Vec<Hub<Mutation>>,
    #[builder(default, setter(each(name = "entry", into)))]
    entries: Vec<Entry>,
}

impl Render {
    async fn render(&self, post: &mut Vec<post::Render>) -> graph::Result<()> {
        for cmd in &self.entries {
            match cmd {
                Entry::Pipe(pipe) => post.push(post::Render::Pipe(pipe.clone())),
                Entry::Bind(index, bind) => {
                    let bind = bind.base().await?;
                    post
                        .push(post::Render::Bind(*index, bind.clone()))
                }
                Entry::Vertex(slot, buffer) => {
                    let buffer = buffer.base().await?;
                    post
                        .push(post::Render::Vertex(*slot, buffer.clone()))
                }
                Entry::Index(buffer) => {
                    let buffer = buffer.base().await?;
                    post.push(post::Render::Index(buffer.clone()))
                    // pass.set_index_buffer(buffer.slice(..), IndexFormat::Uint16);
                }
                Entry::Draw(vertices, instances) => {
                    post.push(post::Render::Draw(
                        vertices.clone(),
                        instances.clone(),
                    ))
                    // pass.draw(vertices.clone(), instances.clone());
                }
                Entry::DrawIndexed(indices, base_vertex, instances) => {
                    post.push(post::Render::DrawIndexed(
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

impl Solve for Render {
    type Base = Vec<post::Render>;
    async fn solve(&self) -> graph::Result<Hub<Vec<post::Render>>> {
        // TODO: should not depend on these roots here. instead,
        // they should included in some RenderPass struct along with Vec<post::Render>
        self.roots.depend().await?;
        let mut post = vec![];
        // let mut post = post::Command {
        //     msaa: 4,
        //     ..Default::default()
        // };
        self.render(&mut post).await?;
        // TODO: turn chain.write into a trait function on Leaf<Vec<Command>>>
        // self.chain.write(|chain| chain.push(post)).await?;
        Ok(post.into()) // Hub::Tray(Tray::Base(post))
    }
}

impl Adapt for Render {
    fn back(&mut self, back: &Back) -> graph::Result<()> {
        for cmd in &mut self.entries {
            if let Entry::Vertex(_, buffer) = cmd {
                buffer.back(back)?
            }
        }
        self.roots.back(back)
    }
}

impl CommandBuilder {
    pub fn render(self, pipe: Grc<RenderPipeline>) -> Self {
        self.entry(Entry::Pipe(pipe))
    }
    pub fn bind(self, index: u32, bind: impl Into<Hub<Grc<BindGroup>>>) -> Self {
        self.entry(Entry::Bind(index, bind.into()))
    }
    pub fn vertex(self, slot: u32, buffer: impl Into<Hub<Grc<Buffer>>>) -> Self {
        self.entry(Entry::Vertex(slot, buffer.into()))
    }
    pub fn index(self, buffer: impl Into<Hub<Grc<Buffer>>>) -> Self {
        self.entry(Entry::Index(buffer.into()))
    }
    pub fn draw(self, vertices: Range<u32>, instances: Range<u32>) -> Self {
        self.entry(Entry::Draw(vertices, instances))
    }
    pub fn draw_indexed(
        self,
        indices: Range<u32>,
        base_vertex: i32,
        instances: Range<u32>,
    ) -> Self {
        self.entry(Entry::DrawIndexed(indices, base_vertex, instances))
    }
}

// pub struct RenderPass {
//     command: CommandBuilder,
// }

// impl RenderPass {
//     pub fn hub(self) -> graph::Result<Hub<Mutation>> {
//         self.command.hub()
//     }
//     pub fn bind(mut self, index: u32, bind: impl Into<Hub<Grc<BindGroup>>>) -> Self {
//         self.command = self.command.render_entry(Entry::Bind(index, bind.into()));
//         self
//     }
//     pub fn vertex(mut self, slot: u32, buffer: impl Into<Hub<Grc<Buffer>>>) -> Self {
//         self.command = self
//             .command
//             .render_entry(Entry::Vertex(slot, buffer.into()));
//         self
//     }
//     pub fn index(mut self, buffer: impl Into<Hub<Grc<Buffer>>>) -> Self {
//         self.command = self.command.render_entry(Entry::Index(buffer.into()));
//         self
//     }
//     pub fn draw(mut self, vertices: Range<u32>, instances: Range<u32>) -> Self {
//         self.command = self.command.render_entry(Entry::Draw(vertices, instances));
//         self
//     }
//     pub fn draw_indexed(
//         mut self,
//         indices: Range<u32>,
//         base_vertex: i32,
//         instances: Range<u32>,
//     ) -> Self {
//         self.command =
//             self.command
//                 .render_entry(Entry::DrawIndexed(indices, base_vertex, instances));
//         self
//     }
// }

#[derive(Debug)]
enum Entry {
    Pipe(Grc<RenderPipeline>),
    Bind(u32, Hub<Grc<BindGroup>>),
    // TODO: take Hedge so hedge.root.depend is automatic 
    Vertex(u32, Hub<Grc<Buffer>>),
    Index(Hub<Grc<Buffer>>),
    Draw(Range<u32>, Range<u32>),
    DrawIndexed(Range<u32>, i32, Range<u32>),
}
