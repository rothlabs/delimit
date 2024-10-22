use super::*;

#[derive(Builder, Debug, Gate)]
#[builder(pattern = "owned")]
#[builder(setter(into, strip_option))]
pub struct Compute {
    gpu: Gpu,
    #[builder(default)]
    root: Option<Hub<Mutation>>,
    #[builder(setter(each(name = "cmd", into)))]
    commands: Vec<Command>,
}

impl ComputeBuilder {
    pub fn pipe(self, pipe: Grc<ComputePipeline>) -> Self {
        self.cmd(Command::Pipe(pipe))
    }
    pub fn bind(self, bind: impl Into<Hub<Grc<BindGroup>>>) -> Self {
        self.cmd(Command::Bind(bind.into()))
    }
    pub fn dispatch(self, count: impl Into<Hub<u32>>) -> Self {
        self.cmd(Command::Dispatch(count.into()))
    }
}

impl Solve for Compute {
    type Base = Mutation;
    async fn solve(&self) -> graph::Result<Hub<Mutation>> {
        self.root.depend().await?;
        let mut encoder = self.gpu.encoder();
        {
            let mut pass = encoder.compute();
            for cmd in &self.commands {
                match cmd {
                    Command::Pipe(pipe) => pass.set_pipeline(pipe),
                    Command::Bind(bind) => {
                        let bind = bind.base().await?;
                        pass.set_bind_group(0, &bind, &[])
                    },
                    Command::Dispatch(count) => {
                        let count = count.base().await?;
                        pass.dispatch_workgroups(count, 1, 1)
                    },
                }
            }
        }
        encoder.submit();
        Ok(Mutation.into())
    }
}

impl Adapt for Compute {
    fn back(&mut self, back: &Back) -> graph::Result<()> {
        for cmd in &mut self.commands {
            match cmd {
                Command::Bind(bind) => bind.back(back),
                Command::Dispatch(count) => count.back(back),
                Command::Pipe(_) => Ok(())
            }?;
        }
        self.root.back(back)
    }
}

#[derive(Debug)]
enum Command {
    Pipe(Grc<ComputePipeline>),
    Bind(Hub<Grc<BindGroup>>),
    Dispatch(Hub<u32>),
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
