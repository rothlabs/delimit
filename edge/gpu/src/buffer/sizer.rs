use super::*;

#[derive(Builder, Gate, Debug)]
#[builder(pattern = "owned")]
#[builder(setter(into))]
pub struct BufferSizer {
    gpu: Gpu,
    root: Hub<Grc<Buffer>>,
    #[builder(setter(each(name = "mul", into)))]
    muls: Vec<Hub<u32>>,
    #[builder(setter(each(name = "div", into)))]
    divs: Vec<Hub<u32>>,
}

impl Solve for BufferSizer {
    type Base = Grc<Buffer>;
    async fn solve(&self) -> graph::Result<Hub<Grc<Buffer>>> {
        let mut size = self.root.base().await?.size();
        for mul in &self.muls {
            size *= mul.base().await? as u64;
        }
        for div in &self.divs {
            size /= div.base().await? as u64;
        }
        let buffer = self.gpu.buffer(size).storage_copy()?.into();
        Ok(buffer)
    }
}

impl Adapt for BufferSizer {
    fn back(&mut self, back: &Back) -> graph::Result<()> {
        self.root.back(back)?;
        self.muls.back(back)?;
        self.divs.back(back)
    }
}