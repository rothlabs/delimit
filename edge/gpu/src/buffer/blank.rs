use super::*;

#[derive(Builder, Back, Gate, Debug)]
#[builder(pattern = "owned")]
#[builder(setter(into))]
pub struct Blank {
    #[back(skip)]
    core: Core,
    root: Hub<Grc<Buffer>>,
    #[builder(default, setter(each(name = "mul", into)))]
    muls: Vec<Hub<u32>>,
    #[builder(default, setter(each(name = "div", into)))]
    divs: Vec<Hub<u32>>,
    #[back(skip)]
    #[builder(default = "BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST")]
    usage: BufferUsages,
}

impl Solve for Blank {
    type Base = Grc<Buffer>;
    async fn solve(&self) -> graph::Result<Hub<Grc<Buffer>>> {
        let mut size = self.root.base().await?.size();
        for mul in &self.muls {
            size *= mul.base().await? as u64;
        }
        for div in &self.divs {
            size /= div.base().await? as u64;
        }
        Ok(self.core.buffer(size).usage(self.usage).make()?.into())
    }
}

impl BlankBuilder {
    pub fn map_read(self) -> graph::Result<Hub<Grc<Buffer>>> {
        self.usage(BufferUsages::MAP_READ | BufferUsages::COPY_DST)
            .hub()
    }
}
