use super::*;
use std::ops::Range;

#[derive(Builder, Gate, Debug)]
#[builder(pattern = "owned")]
#[builder(setter(into, strip_option))]
pub struct Command {
    // TODO: rename roots to stems
    #[builder(default, setter(each(name = "root", into)))]
    roots: Vec<Hub<Mutation>>,
    // TODO: take enum of either DIRECT(Grc<TextureView>) or RESOLVE(Grc<TextureView>, Grc<TextureView>)
    // #[builder(default)]
    // display: Leaf<Grc<Display>>,
    #[builder(default, setter(each(name = "compute_entry", into)))]
    compute_entries: Vec<compute::Entry>,
    #[builder(default, setter(each(name = "render_entry", into)))]
    render_entries: Vec<Entry>,
    chain: Leaf<Vec<post::Command>>,
}

impl Command {
    async fn compute(&self, post: &mut post::Command) -> graph::Result<()> {
        for cmd in &self.compute_entries {
            match cmd {
                compute::Entry::Pipe(pipe) => {
                    post.compute.push(post::compute::Entry::Pipe(pipe.clone()))
                }
                compute::Entry::Bind(index, bind) => {
                    let bind = bind.base().await?;
                    post.compute
                        .push(post::compute::Entry::Bind(*index, bind.clone()))
                }
                compute::Entry::Dispatch(count) => {
                    let count = count.base().await?;
                    post.compute.push(post::compute::Entry::Dispatch(count))
                }
            }
        }
        Ok(())
    }
    async fn render(&self, post: &mut post::Command) -> graph::Result<()> {
        for cmd in &self.render_entries {
            match cmd {
                Entry::Pipe(pipe) => post.render.push(post::render::Entry::Pipe(pipe.clone())),
                Entry::Bind(index, bind) => {
                    let bind = bind.base().await?;
                    post.render
                        .push(post::render::Entry::Bind(*index, bind.clone()))
                }
                Entry::Vertex(slot, buffer) => {
                    let buffer = buffer.base().await?;
                    post.render
                        .push(post::render::Entry::Vertex(*slot, buffer.clone()))
                }
                Entry::Index(buffer) => {
                    let buffer = buffer.base().await?;
                    post.render.push(post::render::Entry::Index(buffer.clone()))
                    // pass.set_index_buffer(buffer.slice(..), IndexFormat::Uint16);
                }
                Entry::Draw(vertices, instances) => {
                    post.render.push(post::render::Entry::Draw(
                        vertices.clone(),
                        instances.clone(),
                    ))
                    // pass.draw(vertices.clone(), instances.clone());
                }
                Entry::DrawIndexed(indices, base_vertex, instances) => {
                    post.render.push(post::render::Entry::DrawIndexed(
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

impl Solve for Command {
    type Base = Mutation;
    async fn solve(&self) -> graph::Result<Hub<Mutation>> {
        self.roots.depend().await?;
        let mut post = post::Command::default();
        post.msaa = 4;
        self.compute(&mut post).await?;
        self.render(&mut post).await?;
        // TODO: turn chain.write into a trait function on Leaf<Vec<Command>>>
        self.chain.write(|chain| {
            if let Some(cmd) = chain.last() {
                post.number = cmd.number + 1;
            }
            chain.push(post);
        }).await?;
        Ok(Mutation.into())
    }
}

impl Adapt for Command {
    fn back(&mut self, back: &Back) -> graph::Result<()> {
        for cmd in &mut self.compute_entries {
            match cmd {
                compute::Entry::Bind(_, bind) => bind.back(back)?,
                compute::Entry::Dispatch(count) => count.back(back)?,
                _ => (),
            }
        }
        for cmd in &mut self.render_entries {
            if let Entry::Vertex(_, buffer) = cmd {
                buffer.back(back)?
            }
        }
        self.roots.back(back)
    }
}

impl CommandBuilder {
    pub fn compute(self, pipe: Grc<ComputePipeline>) -> ComputePass {
        let command = self.compute_entry(compute::Entry::Pipe(pipe));
        ComputePass { command }
    }
    pub fn render(self, pipe: Grc<RenderPipeline>) -> RenderPass {
        let command = self.render_entry(Entry::Pipe(pipe));
        RenderPass { command }
    }
}

pub struct ComputePass {
    command: CommandBuilder,
}

impl ComputePass {
    pub fn hub(self) -> graph::Result<Hub<Mutation>> {
        self.command.hub()
    }
    pub fn bind(mut self, index: u32, bind: impl Into<Hub<Grc<BindGroup>>>) -> Self {
        self.command = self
            .command
            .compute_entry(compute::Entry::Bind(index, bind.into()));
        self
    }
    pub fn dispatch(mut self, count: impl Into<Hub<u32>>) -> Self {
        self.command = self
            .command
            .compute_entry(compute::Entry::Dispatch(count.into()));
        self
    }
    pub fn render(self, pipe: Grc<RenderPipeline>) -> RenderPass {
        self.command.render(pipe)
    }
}

pub struct RenderPass {
    command: CommandBuilder,
}

impl RenderPass {
    pub fn hub(self) -> graph::Result<Hub<Mutation>> {
        self.command.hub()
    }
    pub fn bind(mut self, index: u32, bind: impl Into<Hub<Grc<BindGroup>>>) -> Self {
        self.command = self.command.render_entry(Entry::Bind(index, bind.into()));
        self
    }
    pub fn vertex(mut self, slot: u32, buffer: impl Into<Hub<Grc<Buffer>>>) -> Self {
        self.command = self
            .command
            .render_entry(Entry::Vertex(slot, buffer.into()));
        self
    }
    pub fn index(mut self, buffer: impl Into<Hub<Grc<Buffer>>>) -> Self {
        self.command = self.command.render_entry(Entry::Index(buffer.into()));
        self
    }
    pub fn draw(mut self, vertices: Range<u32>, instances: Range<u32>) -> Self {
        self.command = self.command.render_entry(Entry::Draw(vertices, instances));
        self
    }
    pub fn draw_indexed(
        mut self,
        indices: Range<u32>,
        base_vertex: i32,
        instances: Range<u32>,
    ) -> Self {
        self.command =
            self.command
                .render_entry(Entry::DrawIndexed(indices, base_vertex, instances));
        self
    }
}

#[derive(Debug)]
enum Entry {
    Pipe(Grc<RenderPipeline>),
    Bind(u32, Hub<Grc<BindGroup>>),
    Vertex(u32, Hub<Grc<Buffer>>),
    Index(Hub<Grc<Buffer>>),
    Draw(Range<u32>, Range<u32>),
    DrawIndexed(Range<u32>, i32, Range<u32>),
}
