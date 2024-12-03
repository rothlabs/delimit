use super::*;

#[derive(Debug, Back, Gate)]
pub struct Size {
    pub buffer: Hub<Grc<Buffer>>,
}

impl Size {
    pub fn new(buffer: impl Into<Hub<Grc<Buffer>>>) -> Self {
        Self {
            buffer: buffer.into(),
        }
    }
}

impl Solve for Size {
    type Base = u32;
    async fn solve(&self) -> node::Result<u32> {
        let size = (self.buffer.base().await?.size() / 4) as u32;
        Ok(size.into())
    }
}

// #[derive(Builder, Back, BuildGate, Debug, Make)]
// #[builder(pattern = "owned")]
// #[builder(setter(into))]
