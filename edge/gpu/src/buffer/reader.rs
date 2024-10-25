use super::*;

#[derive(Builder, Back, Gate, Debug)]
#[builder(pattern = "owned")]
#[builder(setter(into, strip_option))]
pub struct BufferReader<T> {
    #[back(skip)]
    gpu: Gpu,
    root: Hub<Mutation>,
    storage: Hub<Grc<Buffer>>,
    stage: Hub<Grc<Buffer>>,
    #[builder(default)]
    #[back(skip)]
    phantom: std::marker::PhantomData<T>,
}

impl<T> Solve for BufferReader<T>
where
    T: Pod,
{
    type Base = Vec<T>;
    async fn solve(&self) -> graph::Result<Hub<Vec<T>>> {
        self.root.base().await?;
        let storage = self.storage.base().await?;
        let stage = self.stage.base().await?;
        self.gpu
            .encoder()
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
        Ok(cast_slice(&data).to_vec().into_leaf().hub())
    }
}

impl<T> BufferReaderBuilder<T>
where
    T: 'static + Clone + Debug,
    BufferReader<T>: Solve,
    <BufferReader<T> as Solve>::Base: Clone + Debug,
{
    pub async fn staged(self) -> graph::Result<Hub<<BufferReader<T> as Solve>::Base>> {
        if let Some(storage) = &self.storage {
            let size = storage.base().await?.size();
            if let Some(gpu) = &self.gpu {
                let stage = gpu.buffer(size).map_read()?;
                return self.stage(stage).hub();
            }
        }
        Err(anyhow!("uninitialized storage"))?
    }
}
