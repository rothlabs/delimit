use super::*;

#[derive(Debug, Clone)]
pub struct Bank {
    pub topic: Hub<stable::GroupBind>,
    pub image: Hub<stable::GroupBind>,
}

impl Bank {
    pub fn new(topic: &Hub<Grc<BindGroup>>, image: &Hub<Grc<BindGroup>>) -> Self {
        let topic = gpu::active::group::Bind {
            slot: 0.into(),
            group: topic.clone(),
            offsets: vec![],
        }
        .hub();
        let image = gpu::active::group::Bind {
            slot: 0.into(),
            group: image.clone(),
            offsets: vec![],
        }
        .hub();
        Self { topic, image }
    }
}

// fn entry(buffer: &Grc<Buffer>) -> BindGroupEntry {
//     BindGroupEntry {
//         binding: 0,
//         resource: BindingResource::Buffer(BufferBinding {
//             buffer,
//             offset: 0,
//             size: Some(NonZero::new(256).unwrap()),
//         }),
//     }
// }

// fn entry(buffer: &Grc<Buffer>) -> BindGroupEntry {
//     BindGroupEntry {
//         binding: 0,
//         resource: buffer.as_entire_binding(),
//     }
// }
