use super::*;

// const LABEL: &str = "gpu_store_rig";

#[derive(Debug)]
pub struct Rig {
    // pub layout: BindGroupLayout,
    // TODO: needs to be hub so only need to change buffer
    // pub group: Leaf<Grc<BindGroup>>,
    pub buffer: Leaf<Grc<Buffer>>,
    chunks: Leaf<Vec<Chunk>>,
}

impl Rig {
    pub fn new(device: &Device) -> Self {
        let buffer = storage_buffer(device);
        // let layout = layout(device);
        // let group = device.create_bind_group(&BindGroupDescriptor {
        //     label: Some(LABEL),
        //     layout: &layout,
        //     entries: &[entry(&buffer)],
        // });
        Self {
            // layout,
            // group: Leaf::new(Grc::new(group)),
            buffer: Leaf::new(buffer),
            chunks: Leaf::default(),
        }
    }
    // TODO: accept number of blocks of 64 elements
    pub fn grant(&self) -> Result<Leaf<u32>> {
        let max = (self.buffer.base()?.size() / 4) as u32;
        grant(&self.chunks, 64, max)
    }
}

// fn entry(buffer: &Grc<Buffer>) -> BindGroupEntry {
//     BindGroupEntry {
//         binding: 0,
//         resource: BindingResource::Buffer(BufferBinding {
//             buffer,
//             offset: 0,
//             size: NonZero::new(256),
//         }),
//     }
// }

// fn layout(device: &Device) -> BindGroupLayout {
//     device.create_bind_group_layout(&BindGroupLayoutDescriptor {
//         label: Some(LABEL),
//         entries: &[layout_entry()],
//     })
// }

// fn layout_entry() -> BindGroupLayoutEntry {
//     BindGroupLayoutEntry {
//         binding: 0,
//         visibility: ShaderStages::COMPUTE | ShaderStages::VERTEX,
//         ty: BindingType::Buffer {
//             ty: BufferBindingType::Storage { read_only: true },
//             has_dynamic_offset: true,
//             min_binding_size: None,
//         },
//         count: None,
//     }
// }
