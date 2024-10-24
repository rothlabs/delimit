use super::*;
use std::ops::Range;

#[derive(Builder, Debug, Gate)]
#[builder(pattern = "owned")]
#[builder(setter(into, strip_option))]
pub struct Command {
    gpu: Gpu,
    #[builder(default, setter(each(name = "root", into)))]
    roots: Vec<Hub<Mutation>>,
    #[builder(default)]
    texture_view: Option<Grc<TextureView>>,
    #[builder(default)]
    resolve_target: Option<Grc<TextureView>>,
    #[builder(default, setter(each(name = "compute_command", into)))]
    compute_commands: Vec<ComputeCommand>,
    #[builder(default, setter(each(name = "render_command", into)))]
    render_commands: Vec<RenderCommand>,
}

impl CommandBuilder {
    pub fn compute(self, pipe: Grc<ComputePipeline>) -> Self {
        self.compute_command(ComputeCommand::Pipe(pipe))
    }
    pub fn bind(self, bind: impl Into<Hub<Grc<BindGroup>>>) -> Self {
        self.compute_command(ComputeCommand::Bind(bind.into()))
    }
    pub fn dispatch(self, count: impl Into<Hub<u32>>) -> Self {
        self.compute_command(ComputeCommand::Dispatch(count.into()))
    }
    pub fn render(self, pipe: Grc<RenderPipeline>) -> Self {
        self.render_command(RenderCommand::Pipe(pipe))
    }
    pub fn vertex(self, slot: u32, buffer: impl Into<Hub<Grc<Buffer>>>) -> Self {
        self.render_command(RenderCommand::Vertex((slot, buffer.into())))
    }
    pub fn draw(self, vertices: Range<u32>, instances: Range<u32>) -> Self {
        self.render_command(RenderCommand::Draw((vertices, instances)))
    }
}

impl Command {
    async fn compute_pass(&self, encoder: &mut Encoder<'_>) -> graph::Result<()> {
        let mut pass = encoder.compute();
        for cmd in &self.compute_commands {
            match cmd {
                ComputeCommand::Pipe(pipe) => pass.set_pipeline(pipe),
                ComputeCommand::Bind(bind) => {
                    let bind = bind.base().await?;
                    pass.set_bind_group(0, &bind, &[])
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
        encoder: &mut Encoder<'_>,
        view: &TextureView,
    ) -> graph::Result<()> {
        let attachments = if let Some(target) = &self.resolve_target {
            self.gpu
                .attachment(view)
                .resolve_target(target)
                .list()?
        } else {
            self.gpu.attachment(view).list()?
        };
        let render = self.gpu.render_pass(&attachments).make()?;
        let mut pass = encoder.render(&render);
        for cmd in &self.render_commands {
            match cmd {
                RenderCommand::Pipe(pipe) => pass.set_pipeline(pipe),
                RenderCommand::Vertex((slot, buffer)) => {
                    let buffer = buffer.base().await?;
                    pass.set_vertex_buffer(*slot, buffer.slice(..));
                }
                RenderCommand::Draw((vertices, instances)) => {
                    pass.draw(vertices.clone(), instances.clone());
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
        let mut encoder = self.gpu.encoder();
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
                ComputeCommand::Bind(bind) => bind.back(back)?,
                ComputeCommand::Dispatch(count) => count.back(back)?,
                _ => (),
            }
        }
        for cmd in &mut self.render_commands {
            if let RenderCommand::Vertex((_, buffer)) = cmd {
                buffer.back(back)?
            }
        }
        self.roots.back(back)
    }
}

#[derive(Debug)]
enum ComputeCommand {
    Pipe(Grc<ComputePipeline>),
    Bind(Hub<Grc<BindGroup>>),
    Dispatch(Hub<u32>),
}

#[derive(Debug)]
enum RenderCommand {
    Pipe(Grc<RenderPipeline>),
    Vertex((u32, Hub<Grc<Buffer>>)),
    Draw((Range<u32>, Range<u32>)),
}
