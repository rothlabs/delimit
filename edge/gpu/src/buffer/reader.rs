use super::*;

#[derive(Builder, Gate, Debug)]
#[builder(pattern = "owned")]
#[builder(setter(into, strip_option))]
pub struct BufferReader<T> {
    gpu: Gpu,
    mutator: Hub<Mutation>,
    storage: Hub<Grc<Buffer>>,
    stage: Hub<Grc<Buffer>>,
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
        let storage = self.storage.base().await?;
        let stage = self.stage.base().await?;
        self.gpu.encoder()
            .copy_buffer(&storage)
            .destination(&stage)
            .size(stage.size())
            .submit();
        let slice = stage.slice(..);
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
        self.storage.back(back)?;
        self.stage.back(back)
    }
}

// staging: Option<Hub<Grc<Buffer>>>,

// impl<T> BufferReaderBuilder<T> {
//     pub fn stage(
//         self,
//         storage: impl Into<Hub<Grc<Buffer>>>,
//         stage: impl Into<Hub<Grc<Buffer>>>,
//     ) -> Self {
//         self.staging((storage.into(), stage.into()))
//     }
// }
