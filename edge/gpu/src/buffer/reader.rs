use super::*;
use std::marker::PhantomData;

#[derive(Builder, Debug)] // , Input!
#[builder(pattern = "owned")]
#[builder(setter(into))]
pub struct BufferReader<T> {
    // buffer: Grc<wgpu::Buffer>,
    buffer: Hub<crate::Buffer>,
    #[builder(default, setter(each(name = "stem", into)))]
    stems: Vec<Apex>,
    #[builder(default)]
    phantom: PhantomData<T>,
}


impl<T> GateTag for BufferReader<T> {}

impl<T> BufferReaderBuilder<T>
where
    T: 'static + Clone + Debug,
    BufferReader<T>: Solve,
    <BufferReader<T> as Solve>::Base: Clone + Debug,
{
    pub fn make(self) -> graph::Result<BufferReader<T>> {
        match self.build() {
            Ok(value) => Ok(value),
            Err(err) => Err(anyhow!(err.to_string()))?,
        }
    }
    pub fn node(self) -> graph::Result<Node<BufferReader<T>>> {
        self.make()?.node()
    }
    pub fn hub(self) -> graph::Result<Hub<<BufferReader<T> as Solve>::Base>> {
        Ok(self.make()?.gate()?.into())
    }
}

impl<T> Solve for BufferReader<T>
where
    T: Pod,
{
    type Base = Vec<T>;
    async fn solve(&self) -> graph::Result<Hub<Vec<T>>> {
        self.stems.depend().await?;
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
    fn adapt(&mut self, deal: &mut dyn Deal) -> graph::Result<()> {
        self.stems.deal("stems", deal)
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
//         let hub = node.gate()?;
//         Ok(hub.into())
//     }
// }
