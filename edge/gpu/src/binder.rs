use super::*;

#[derive(Builder, Debug, Gate)]
#[builder(pattern = "owned")]
#[builder(setter(into, strip_option))]
// #[builder(setter(strip_option))]
pub struct Binder {
    gpu: Gpu,
    #[builder(default)]
    layout: Option<Grc<BindGroupLayout>>,
    #[builder(default)]
    pipe: Option<Grc<ComputePipeline>>,
    #[builder(default, setter(each(name = "inner_entry", into)))]
    buffers: Vec<(u32, Hub<Grc<Buffer>>)>,
}

impl BinderBuilder {
    pub fn entry(self, i: u32, buffer: impl Into<Hub<Grc<Buffer>>>) -> Self {
        self.inner_entry((i, buffer.into()))
    }
}

impl Solve for Binder {
    type Base = Grc<BindGroup>;
    async fn solve(&self) -> graph::Result<Hub<Self::Base>> {
        let mut bind = self.gpu.bind();
        let mut buffers = vec![];
        for (i, buffer) in &self.buffers {
            buffers.push((i, buffer.base().await?));
        }
        for (i, buffer) in &buffers {
            bind = bind.entry(**i, buffer);
        }
        if let Some(layout) = &self.layout {
            bind = bind.layout(layout);
        } else if let Some(pipe) = &self.pipe {
            bind = bind.pipe(pipe);
        }
        Ok(bind.make()?.into())
    }
}

impl Adapt for Binder {
    fn back(&mut self, back: &Back) -> graph::Result<()> {
        for (_, buffer) in &mut self.buffers {
            buffer.back(back)?;
        }
        Ok(())
    }
}

// impl Solve for Binder {
//     type Base = Grc<BindGroup>;
//     async fn solve(&self) -> graph::Result<Hub<Self::Base>> {
//         let mut bind = self.gpu.bind();
//         let mut buffers = vec![];
//         for (i, buffer) in &self.buffers {
//             buffers.push((i, buffer.base().await?));
//         }
//         for (i, buffer) in &buffers {
//             bind = bind.entry(**i, buffer);
//         }
//         if let Some(layout) = &self.layout {
//             //let layout = layout.base().await?;
//             Ok(bind.layout(layout).make()?.into())
//         } else if let Some(pipe) = &self.pipe {
//             //let pipe = pipe.base().await?;
//             Ok(bind.pipe(&pipe).make()?.into())
//         } else {
//             Ok(bind.make()?.into())
//         }
//     }
// }

// impl Adapt for Binder {
//     fn back(&mut self, back: &Back) -> graph::Result<()> {
//         self.layout.back(back)?;
//         self.pipe.back(back)?;
//         for (_, buffer) in &mut self.buffers {
//             buffer.back(back)?;
//         }
//         Ok(())
//     }
// }
