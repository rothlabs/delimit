use super::*;

#[derive(Builder, Debug, Gate)]
#[builder(pattern = "owned")]
#[builder(setter(into, strip_option))]
pub struct Dispatcher {
    gpu: Gpu,
    #[builder(default)]
    root: Option<Hub<Mutation>>,
    pipe: Grc<ComputePipeline>,
    bind: Hub<Grc<BindGroup>>,
    count: Hub<u32>,
}

impl Solve for Dispatcher {
    type Base = Mutation;
    async fn solve(&self) -> graph::Result<Hub<Mutation>> {
        self.root.depend().await?;
        let bind = self.bind.base().await?;
        let count = self.count.base().await?;
        let mut encoder = self.gpu.encoder();
        encoder
            .compute()
            .pipe(&self.pipe)
            .bind(0, &bind, &[])
            .dispatch(count, 1, 1);
        encoder.submit();
        Ok(Mutation.into())
    }
}

impl Adapt for Dispatcher {
    fn back(&mut self, back: &Back) -> graph::Result<()> {
        self.root.back(back)?;
        self.bind.back(back)?;
        self.count.back(back)
    }
}


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

// impl DispatcherBuilder {
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
