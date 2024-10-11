use super::*;

#[derive(Builder, Debug, Unit!)]
#[builder(pattern = "owned")]
#[builder(setter(into))]
pub struct Dispatcher {
    gpu: Gpu,
    pipe: Leaf<ComputePipeline>,
    bind: Leaf<BindGroup>,
    count: Hub<u32>,
    stage: Option<(Leaf<crate::Buffer>, Leaf<crate::Buffer>)>,
}

impl Act for Dispatcher {
    async fn act(&self) -> graph::Result<()> {
        let count = self.count.base().await?;
        self.pipe.read(|pipe| {
            self.bind.read(|bind| {
                let mut encoder = self.gpu.encoder();
                encoder
                    .compute()
                    .pipe(pipe)
                    .bind(0, bind, &[])
                    .dispatch(count, 1, 1);
                if let Some((storage, stage)) = &self.stage {
                    storage.read(|storage| {
                        stage.read(|stage| {
                            encoder
                                .copy_buffer(storage)
                                .destination(stage)
                                .size(4 * count as u64)
                                .submit();
                        })
                    })??;
                } else {
                    encoder.submit();
                }
                Ok(())
            })?
        })?
    }
}

impl Adapt for Dispatcher {
    fn back(&mut self, back: &Back) -> graph::Result<()> {
        self.pipe.back(back)?;
        self.bind.back(back)?;
        self.count.back(back)
        // self.stage.b
    }
}


// self.stems.depend().await?;
// #[builder(default, setter(each(name = "stem", into)))]
//     stems: Vec<Apex>,