use super::*;

#[derive(Builder, Back, Gate, Debug)]
#[builder(pattern = "owned")]
#[builder(setter(into))]
pub struct Size {
    #[back(skip)]
    core: Core,
    buffer: Hub<Grc<Buffer>>,
    #[builder(default, setter(each(name = "mul", into)))]
    muls: Vec<Hub<u32>>,
    #[builder(default, setter(each(name = "div", into)))]
    divs: Vec<Hub<u32>>,
}

impl Solve for Size {
    type Base = u64;
    async fn solve(&self) -> graph::Result<Hub<u64>> {
        let mut size = self.buffer.base().await?.size();
        for mul in &self.muls {
            size *= mul.base().await? as u64;
        }
        for div in &self.divs {
            size /= div.base().await? as u64;
        }
        Ok(size.into())
    }
}
