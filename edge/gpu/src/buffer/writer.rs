use super::*;

#[derive(Builder, Back, BuildGate, Debug)]
#[builder(pattern = "owned", setter(into))]
pub struct BufferWriter<T> {
    #[back(skip)]
    pub queue: Grc<Queue>,
    pub buffer: Hub<Grc<Buffer>>,
    pub data: Hub<Vec<T>>,
    #[builder(default)]
    pub index: Hub<u32>,
}

impl<T> Solve for BufferWriter<T>
where
    T: Pod + Debug + graph::SendSync,
{
    type Base = Grc<stable::Command>;
    async fn solve(&self) -> node::Result<Grc<stable::Command>> {
        let buffer = self.buffer.base().await?;
        let offset = self.index.base().await.unwrap_or_default() as u64 * 4;
        self.data
            .read(|data| {
                self.queue.write_buffer(&buffer, offset, cast_slice(data));
            })
            .await?;
        Ok(Grc::new(stable::Command::default()).into())
    }
}

// impl<T> Adapt for BufferWriter<T>
// where
//     T: 'static + Clone,
// {
//     fn back(&mut self, back: &Back) -> graph::Result<()> {
//         self.offset.back(back)?;
//         self.data.back(back)
//     }
// }

// let size = NonZero::new(buffer.size()).unwrap();
//                 let mut view = self.queue.write_buffer_with(&buffer, offset, size);
//                 if let Some(view) = &mut view {
//                     let mut wow = view.deref_mut();
//                     //let mut crap = cast_slice::<T, u8>(data);
//                     let mut crap = bytemuck::cast_mut::<T, u8>(data);
//                     wow = &mut crap;
//                     Ok::<(), graph::Error>(())
//                 } else {
//                     Err(anyhow!("could not write to buffer"))?
//                 }

// let slice = buffer.slice(..);
//         let (sender, receiver) = flume::bounded(1);
//         self.data
//             .read(|data| {
//                 //self.queue.write_buffer(&buffer, offset, cast_slice(data));
//                 // TODO: remove this unwrap

//                 slice.map_async(wgpu::MapMode::Write, move |v| sender.send(v).unwrap());

//             })
//             .await?;
//         if let Err(err) = receiver.recv_async().await? {
//             return Err(anyhow!(err))?;
//         }
//         let mut data = slice.get_mapped_range_mut();
//         let crap: &mut [T] = bytemuck::cast_slice_mut(&mut data);
//         self.data
//             .write(|data| {
//                 crap = data.as_slice_mut();
//         }).await?;

// #[derive(Builder, Gate, Debug)]
// #[builder(pattern = "owned")]
// #[builder(setter(into))]
// pub struct BufferWriter<T> {
//     queue: Grc<wgpu::Queue>,
//     buffer: Hub<Grc<Buffer>>,
//     #[builder(default)]
//     offset: Hub<u64>,
//     data: Hub<Vec<T>>,
// }

// impl<T> Solve for BufferWriter<T>
// where
//     T: Pod + Debug,
// {
//     type Base = Mutation;
//     async fn solve(&self) -> graph::Result<Hub<Mutation>> {
//         let buffer = self.buffer.base().await?;
//         let offset = self.offset.base().await.unwrap_or_default();
//         self.data
//             .read(|data| {
//                 self.queue.write_buffer(&buffer, offset, cast_slice(data));
//             })
//             .await?;
//         Ok(Mutation {}.into())
//     }
// }

// impl<T> Adapt for BufferWriter<T>
// where
//     T: 'static + Clone,
// {
//     fn back(&mut self, back: &Back) -> graph::Result<()> {
//         self.offset.back(back)?;
//         self.data.back(back)
//     }
// }
