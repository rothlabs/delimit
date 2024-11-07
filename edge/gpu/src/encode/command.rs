use super::*;
use std::ops::Range;

#[derive(Builder, Gate, Debug)]
#[builder(pattern = "owned")]
#[builder(setter(into, strip_option))]
pub struct Command {
    core: Core,
    #[builder(default, setter(each(name = "root", into)))]
    roots: Vec<Hub<Mutation>>,
    // TODO: take enum of either DIRECT(Grc<TextureView>) or RESOLVE(Grc<TextureView>, Grc<TextureView>)
    #[builder(default)]
    texture_view: Option<Grc<TextureView>>,
    #[builder(default)]
    resolve_target: Option<Grc<TextureView>>,
    #[builder(default, setter(each(name = "compute_command", into)))]
    compute_commands: Vec<ComputeCommand>,
    #[builder(default, setter(each(name = "render_command", into)))]
    render_commands: Vec<RenderCommand>,
}

impl Command {
    async fn compute_pass(&self, encoder: &mut Encode<'_>) -> graph::Result<()> {
        let mut pass = encoder.compute();
        for cmd in &self.compute_commands {
            match cmd {
                ComputeCommand::Pipe(pipe) => pass.set_pipeline(pipe),
                ComputeCommand::Bind(index, bind) => {
                    let bind = bind.base().await?;
                    pass.set_bind_group(*index, &bind, &[])
                }
                ComputeCommand::Dispatch(count) => {
                    let count = count.base().await?;
                    pass.dispatch_workgroups(count, 1, 1)
                }
            }
        }
        Ok(())
    }
    async fn render_pass(
        &self,
        encoder: &mut Encode<'_>,
        view: &TextureView,
    ) -> graph::Result<()> {
        let attachments = if let Some(target) = &self.resolve_target {
            self.core.attachment(view).resolve_target(target).list()?
        } else {
            self.core.attachment(view).list()?
        };
        let render = self.core.render_pass(&attachments).make()?;
        let mut pass = encoder.render(&render);
        for cmd in &self.render_commands {
            match cmd {
                RenderCommand::Pipe(pipe) => pass.set_pipeline(pipe),
                RenderCommand::Bind(index, bind) => {
                    let bind = bind.base().await?;
                    pass.set_bind_group(*index, &bind, &[])
                }
                RenderCommand::Vertex(slot, buffer) => {
                    let buffer = buffer.base().await?;
                    pass.set_vertex_buffer(*slot, buffer.slice(..));
                }
                RenderCommand::Index(buffer) => {
                    let buffer = buffer.base().await?;
                    pass.set_index_buffer(buffer.slice(..), IndexFormat::Uint16);
                }
                RenderCommand::Draw(vertices, instances) => {
                    pass.draw(vertices.clone(), instances.clone());
                }
                RenderCommand::DrawIndexed((indices, base_vertex, instances)) => {
                    pass.draw_indexed(indices.clone(), *base_vertex, instances.clone());
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
        let mut encoder = self.core.encoder();
        if !self.compute_commands.is_empty() {
            self.compute_pass(&mut encoder).await?;
        }
        if let Some(view) = &self.texture_view {
            self.render_pass(&mut encoder, view).await?;
        }
        encoder.submit();
        Ok(Mutation.into())
    }
}

impl Adapt for Command {
    fn back(&mut self, back: &Back) -> graph::Result<()> {
        for cmd in &mut self.compute_commands {
            match cmd {
                ComputeCommand::Bind(_, bind) => bind.back(back)?,
                ComputeCommand::Dispatch(count) => count.back(back)?,
                _ => (),
            }
        }
        for cmd in &mut self.render_commands {
            if let RenderCommand::Vertex(_, buffer) = cmd {
                buffer.back(back)?
            }
        }
        self.roots.back(back)
    }
}

impl CommandBuilder {
    pub fn compute(self, pipe: Grc<ComputePipeline>) -> ComputePass {
        let command = self.compute_command(ComputeCommand::Pipe(pipe));
        ComputePass { command }
    }
    pub fn render(self, pipe: Grc<RenderPipeline>) -> RenderPass {
        let command = self.render_command(RenderCommand::Pipe(pipe));
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
            .compute_command(ComputeCommand::Bind(index, bind.into()));
        self
    }
    pub fn dispatch(mut self, count: impl Into<Hub<u32>>) -> Self {
        self.command = self
            .command
            .compute_command(ComputeCommand::Dispatch(count.into()));
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
        self.command = self
            .command
            .render_command(RenderCommand::Bind(index, bind.into()));
        self
    }
    pub fn vertex(mut self, slot: u32, buffer: impl Into<Hub<Grc<Buffer>>>) -> Self {
        self.command = self
            .command
            .render_command(RenderCommand::Vertex(slot, buffer.into()));
        self
    }
    pub fn index(mut self, buffer: impl Into<Hub<Grc<Buffer>>>) -> Self {
        self.command = self
            .command
            .render_command(RenderCommand::Index(buffer.into()));
        self
    }
    pub fn draw(mut self, vertices: Range<u32>, instances: Range<u32>) -> Self {
        self.command = self
            .command
            .render_command(RenderCommand::Draw(vertices, instances));
        self
    }
    pub fn draw_indexed(
        mut self,
        indices: Range<u32>,
        base_vertex: i32,
        instances: Range<u32>,
    ) -> Self {
        self.command = self.command.render_command(RenderCommand::DrawIndexed((
            indices,
            base_vertex,
            instances,
        )));
        self
    }
}

#[derive(Debug)]
enum ComputeCommand {
    Pipe(Grc<ComputePipeline>),
    Bind(u32, Hub<Grc<BindGroup>>),
    Dispatch(Hub<u32>),
}

#[derive(Debug)]
enum RenderCommand {
    Pipe(Grc<RenderPipeline>),
    Bind(u32, Hub<Grc<BindGroup>>),
    Vertex(u32, Hub<Grc<Buffer>>),
    Index(Hub<Grc<Buffer>>),
    Draw(Range<u32>, Range<u32>),
    DrawIndexed((Range<u32>, i32, Range<u32>)),
}
