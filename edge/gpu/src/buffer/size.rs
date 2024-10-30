use super::*;

#[derive(Builder, Back, Gate, Debug)]
#[builder(pattern = "owned")]
#[builder(setter(into))]
pub struct Size {
    #[back(skip)]
    core: Core,
    buffer: Hub<Grc<Buffer>>,
    #[builder(default, setter(each(name = "add", into)))]
    adds: Vec<Hub<u32>>,
    #[builder(default, setter(each(name = "mul", into)))]
    muls: Vec<Hub<u32>>,
    #[builder(default, setter(each(name = "div", into)))]
    divs: Vec<Hub<u32>>,
}

impl Solve for Size {
    type Base = u32;
    async fn solve(&self) -> graph::Result<Hub<u32>> {
        let mut size = (self.buffer.base().await?.size() / 4) as u32;
        for add in &self.adds {
            size += add.base().await?;
        }
        for mul in &self.muls {
            size *= mul.base().await?;
        }
        for div in &self.divs {
            size /= div.base().await?;
        }
        Ok(size.into())
    }
}
