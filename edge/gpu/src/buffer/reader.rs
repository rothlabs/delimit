use super::*;

#[derive(Builder, Debug, Gate)]
#[builder(pattern = "owned")]
#[builder(setter(into))]
pub struct BufferReader<T> {
    mutator: Hub<Mutation>,
    buffer: Hub<crate::Buffer>,
    #[builder(default)]
    phantom: std::marker::PhantomData<T>,
}

impl<T> Solve for BufferReader<T>
where
    T: Pod,
{
    type Base = Vec<T>;
    async fn solve(&self) -> graph::Result<Hub<Vec<T>>> {
        self.mutator.base().await?;
        let buffer = self.buffer.base().await?;
        let slice = buffer.slice(..);
        let (sender, receiver) = flume::bounded(1);
        slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
        if let Err(err) = receiver.recv_async().await? {
            return Err(anyhow!(err))?;
        }
        let data = slice.get_mapped_range();
        let out = bytemuck::cast_slice(&data).to_vec();
        Ok(out.into_leaf().hub())
    }
}

impl<T> Adapt for BufferReader<T> {
    fn back(&mut self, back: &Back) -> graph::Result<()> {
        self.mutator.back(back)?;
        self.buffer.back(back)
    }
}

// buffer: Grc<wgpu::Buffer>,

// #[builder(default, setter(each(name = "stem", into)))]
    // stems: Vec<Hub<Mutation>>,

// fn adapt(&mut self, deal: &mut dyn Deal) -> graph::Result<()> {
//     self.stems.deal("stems", deal)
// }

// impl<T> BufferReaderBuilder<T>
// where
//     T: Payload + AnyBitPattern,
//     Vec<T>: Payload,
// {
//     pub fn make(self) -> graph::Result<BufferReader<T>> {
//         match self.build() {
//             Ok(value) => Ok(value),
//             Err(err) => Err(anyhow!(err.to_string()))?,
//         }
//     }
//     pub fn node(self) -> graph::Result<Node<BufferReader<T>>> {
//         self.make()?.node()
//     }
//     pub fn hub(self) -> graph::Result<Hub<Vec<T>>> {
//         let node = self.node()?;
//         let hub = node.gate()?;
//         Ok(hub.into())
//     }
// }
