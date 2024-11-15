use super::*;

#[derive(Builder, Gate, Debug)]
#[builder(pattern = "owned")]
#[builder(setter(into, strip_option))]
pub struct Compute {
    core: Core,
    // TODO: rename roots to stems
    #[builder(default, setter(each(name = "root", into)))]
    roots: Vec<Hub<Mutation>>,
    #[builder(default, setter(each(name = "entry", into)))]
    entries: Vec<Entry>,
}

impl Compute {
    async fn pass(&self, encoder: &mut Encode<'_>) -> graph::Result<()> {
        let mut pass = encoder.compute();
        for cmd in &self.entries {
            match cmd {
                Entry::Pipe(pipe) => pass.set_pipeline(pipe),
                Entry::Bind(index, bind) => {
                    let bind = bind.base().await?;
                    pass.set_bind_group(*index, &bind, &[])
                }
                Entry::Dispatch(count) => {
                    let count = count.base().await?;
                    pass.dispatch_workgroups(count, 1, 1)
                }
            }
        }
        Ok(())
    }
}

impl Solve for Compute {
    type Base = Mutation;
    async fn solve(&self) -> node::Result<Hub<Mutation>> {
        self.roots.depend().await?;
        let mut encoder = self.core.encoder();
        self.pass(&mut encoder).await?;
        encoder.submit();
        Ok(Mutation.into())
    }
}

impl Adapt for Compute {
    fn back(&mut self, back: &Back) -> graph::Result<()> {
        for cmd in &mut self.entries {
            match cmd {
                Entry::Bind(_, bind) => bind.back(back)?,
                Entry::Dispatch(count) => count.back(back)?,
                _ => (),
            }
        }
        self.roots.back(back)
    }
}

impl ComputeBuilder {
    pub fn pipe(self, pipe: Grc<ComputePipeline>) -> Self {
        self.entry(Entry::Pipe(pipe))
    }
    pub fn bind(self, index: u32, bind: impl Into<Hub<Grc<BindGroup>>>) -> Self {
        self.entry(Entry::Bind(index, bind.into()))
    }
    pub fn dispatch(self, count: impl Into<Hub<u32>>) -> Self {
        self.entry(Entry::Dispatch(count.into()))
    }
}

#[derive(Debug)]
pub enum Entry {
    Pipe(Grc<ComputePipeline>),
    Bind(u32, Hub<Grc<BindGroup>>),
    Dispatch(Hub<u32>),
}
