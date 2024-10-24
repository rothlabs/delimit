use super::*;

#[derive(Builder, Gate, Debug)]
#[builder(pattern = "owned")]
#[builder(setter(into, strip_option))]
pub struct Binder {
    gpu: Gpu,
    #[builder(default)]
    layout: Option<Grc<BindGroupLayout>>,
    #[builder(default)]
    pipe: Option<Grc<ComputePipeline>>,
    #[builder(default, setter(each(name = "inner_entry", into)))]
    entries: Vec<(u32, Hub<Grc<Buffer>>)>,
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
        for (i, buffer) in &self.entries {
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
        for (_, buffer) in &mut self.entries {
            buffer.back(back)?;
        }
        Ok(())
    }
}