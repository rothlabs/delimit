use super::*;

#[derive(Builder, Back, Gate, Debug)]
#[builder(pattern = "owned")]
#[builder(setter(into))]
pub struct Blank {
    #[back(skip)]
    gpu: Gpu,
    root: Hub<Grc<Buffer>>,
    #[builder(setter(each(name = "mul", into)))]
    muls: Vec<Hub<u32>>,
    #[builder(setter(each(name = "div", into)))]
    divs: Vec<Hub<u32>>,
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
        Ok(self.gpu.buffer(size).storage_copy()?.into())
    }
}