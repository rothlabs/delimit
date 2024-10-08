use super::*;
use std::marker::PhantomData;

#[derive(Builder, Debug, Input!)] // , Input!
#[builder(pattern = "owned")]
pub struct BufferReader<T> {
    buffer: Grc<wgpu::Buffer>,
    #[builder(default, setter(each(name = "stem", into)))]
    stems: Vec<Apex>,
    #[builder(default)]
    phantom: PhantomData<T>,
}

impl<T> Solve for BufferReader<T>
where
    T: AnyBitPattern,
    Vec<T>: Payload,
{
    type Base = Vec<T>;
    async fn solve(&self) -> graph::Result<Hub<Vec<T>>> {
        self.stems.poll().await?;
        let slice = self.buffer.slice(..);
        let (sender, receiver) = flume::bounded(1);
        slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
        if let Err(err) = receiver.recv_async().await? {
            return Err(anyhow!(err))?;
        }
        let data = slice.get_mapped_range();
        let out = bytemuck::cast_slice(&data).to_vec();
        Ok(out.leaf().hub())
    }
    fn backed(&mut self, back: &Back) -> graph::Result<()> {
        self.stems.back(back)
    }
}

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
//         let hub = node.wing()?;
//         Ok(hub.into())
//     }
// }
