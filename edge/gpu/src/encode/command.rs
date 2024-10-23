use super::*;
use std::ops::Range;

/// TODO: make this a general Encoder node
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
    async fn render_pass(&self, encoder: &mut Encoder<'_>, view: &TextureView) -> graph::Result<()> {
        let attachments = if let Some(resolve_target) = &self.resolve_target {
            self.gpu
                .attachment(view)
                .resolve_target(resolve_target)
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

// pipe: Grc<ComputePipeline>,
// bind: Hub<Grc<BindGroup>>,
// count: Hub<u32>,

//self.commands.back(back)

// impl Backed for Command {
//     fn backed(&self, back: &Back) -> graph::Result<Self> {
//         let backed = match self {
//             Self::Bind(bind) => Self::Bind(bind.backed(back)?),
//             Self::Dispatch(count) => Self::Dispatch(count.backed(back)?),
//             Self::Pipe(pipe) => Self::Pipe(pipe.clone())
//         };
//         Ok(backed)
//     }
// }

// impl BackIt for Command {
//     fn back(&mut self, back: &Back) -> graph::Result<()> {
//         match self {
//             Self::Bind(bind) => bind.back(back),
//             Self::Dispatch(count) => count.back(back),
//             Self::Pipe(_) => Ok(())
//         }
//     }
// }

// impl Solve for Dispatcher {
//     type Base = Mutation;
//     async fn solve(&self) -> graph::Result<Hub<Mutation>> {
//         self.root.depend().await?;
//         let bind = self.bind.base().await?;
//         let count = self.count.base().await?;
//         let mut encoder = self.gpu.encoder();
//         encoder
//             .compute()
//             .pipe(&self.pipe)
//             .bind(0, &bind, &[])
//             .dispatch(count, 1, 1);
//         encoder.submit();
//         Ok(Mutation.into())
//     }
// }

// #[derive(Builder, Debug, Gate)]
// #[builder(pattern = "owned")]
// #[builder(setter(into))]
// pub struct Dispatcher {
//     gpu: Gpu,
//     pipe: Grc<ComputePipeline>,
//     bind: Hub<Grc<BindGroup>>,
//     count: Hub<u32>,
//     #[builder(default, setter(each(name = "mutator", into)))]
//     mutators: Vec<Hub<Mutation>>,
// }

// impl Solve for Dispatcher {
//     type Base = Mutation;
//     async fn solve(&self) -> graph::Result<Hub<Mutation>> {
//         self.mutators.depend().await?;
//         let bind = self.bind.base().await?;
//         let count = self.count.base().await?;
//         let mut encoder = self.gpu.encoder();
//         encoder
//             .compute()
//             .pipe(&self.pipe)
//             .bind(0, &bind, &[])
//             .dispatch(count, 1, 1);
//         encoder.submit();
//         Ok(Mutation.into())
//     }
// }

// impl Adapt for Dispatcher {
//     fn back(&mut self, back: &Back) -> graph::Result<()> {
//         self.mutators.back(back)?;
//         self.count.back(back)
//     }
// }

// staging: Option<(Hub<Grc<Buffer>>, Hub<Grc<Buffer>>)>,
// #[builder(default, setter(each(name = "mutator", into)))]
// mutators: Vec<Hub<Mutation>>,

// if let Some((storage, stage)) = &self.staging {
//     let storage = storage.base().await?;
//     let stage = stage.base().await?;
//     encoder
//         .copy_buffer(&storage)
//         .destination(&stage)
//         .size(4 * count as u64)
//         .submit();
// } else {
//     encoder.submit();
// }

// impl ComputerBuilder {
//     pub fn stage(
//         self,
//         storage: impl Into<Hub<Grc<Buffer>>>,
//         stage: impl Into<Hub<Grc<Buffer>>>,
//     ) -> Self {
//         self.staging((storage.into(), stage.into()))
//     }
// }

// impl Solve for Dispatcher {
//     type Base = Mutation;
//     async fn solve(&self) -> graph::Result<Hub<Mutation>> {
//         self.stems.depend().await?;
//         let count = self.count.base().await?;
//         // self.pipe.read(|pipe| {
//         //     self.bind.read(|bind| {
//         let mut encoder = self.gpu.encoder();
//         encoder
//             .compute()
//             .pipe(&self.pipe)
//             .bind(0, &self.bind, &[])
//             .dispatch(count, 1, 1);
//         if let Some((storage, stage)) = &self.stage {
//             encoder
//                 .copy_buffer(storage)
//                 .destination(stage)
//                 .size(4 * count as u64)
//                 .submit();
//         } else {
//             encoder.submit();
//         }
//         Ok(Mutation{}.into())
//     }
// }

// use super::*;

// #[derive(Builder, Debug, Unit!)]
// #[builder(pattern = "owned")]
// #[builder(setter(into))]
// pub struct Dispatcher {
//     gpu: Gpu,
//     pipe: Leaf<ComputePipeline>,
//     bind: Leaf<BindGroup>,
//     count: Hub<u32>,
//     stage: Option<(Leaf<crate::Buffer>, Leaf<crate::Buffer>)>,
// }

// impl Act for Dispatcher {
//     async fn act(&self) -> graph::Result<()> {
//         let count = self.count.base().await?;
//         self.pipe.read(|pipe| {
//             self.bind.read(|bind| {
//                 let mut encoder = self.gpu.encoder();
//                 encoder
//                     .compute()
//                     .pipe(pipe)
//                     .bind(0, bind, &[])
//                     .dispatch(count, 1, 1);
//                 if let Some((storage, stage)) = &self.stage {
//                     storage.read(|storage| {
//                         stage.read(|stage| {
//                             encoder
//                                 .copy_buffer(storage)
//                                 .destination(stage)
//                                 .size(4 * count as u64)
//                                 .submit();
//                         })
//                     })??;
//                 } else {
//                     encoder.submit();
//                 }
//                 Ok(())
//             })?
//         })?
//     }
// }
