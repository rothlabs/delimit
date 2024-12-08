use super::*;

// const LABEL: &str = "gpu_store_topic";
// const LABEL_VERTEX: &str = "gpu_store_topic_vertex";

#[derive(Debug)]
pub struct Topic {
    // pub layout: BindGroupLayout,
    // pub layout_vertex: BindGroupLayout,
    // pub group: Leaf<Grc<BindGroup>>,
    // pub group_vertex: Leaf<Grc<BindGroup>>,
    pub buffer: Leaf<Grc<Buffer>>,
    // pub bind: Hub<action::GroupBind>,
    // pub bind_vertex: Hub<action::GroupBind>,
    chunks: Leaf<Vec<Chunk>>,
}

impl Topic {
    pub fn new(device: &Device) -> Self {
        let buffer = storage_buffer(device);
        // let layout = layout(device);
        // let layout_vertex = layout_vertex(device);
        // // let group = Leaf::new(Grc::new(device.create_bind_group(&BindGroupDescriptor {
        // //     label: Some(LABEL),
        // //     layout: &layout,
        // //     entries: &[entry(&buffer)],
        // // })));
        // // let bind = Bind::builder().group(&group).build().hub();
        // let group_vertex = Leaf::new(Grc::new(device.create_bind_group(&BindGroupDescriptor {
        //     label: Some(LABEL_VERTEX),
        //     layout: &layout_vertex,
        //     entries: &[entry(&buffer)],
        // })));
        // let bind_vertex = Bind::builder().group(&group_vertex).build().hub();
        Self {
            // layout,
            // layout_vertex,
            // group,        //: Leaf::new(Grc::new(group)),
            // group_vertex, //: Leaf::new(Grc::new(group_vertex)),
            buffer: Leaf::new(buffer),
            // bind,
            // bind_vertex,
            chunks: Leaf::default(),
        }
    }
    pub fn grant(&self, size: u32) -> Result<Leaf<u32>> {
        let max = (self.buffer.base()?.size() / 4) as u32;
        grant(&self.chunks, size, max)
    }
}

// fn entry(buffer: &Grc<Buffer>) -> BindGroupEntry {
//     BindGroupEntry {
//         binding: 0,
//         resource: buffer.as_entire_binding(),
//     }
// }

// fn layout(device: &Device) -> BindGroupLayout {
//     device.create_bind_group_layout(&BindGroupLayoutDescriptor {
//         label: Some(LABEL),
//         entries: &[
//             layout_entry(0, false),
//             // storage_compute_entry(1, true),
//         ],
//     })
// }

// fn layout_entry(binding: u32, read_only: bool) -> BindGroupLayoutEntry {
//     BindGroupLayoutEntry {
//         binding,
//         visibility: ShaderStages::COMPUTE,
//         ty: BindingType::Buffer {
//             ty: BufferBindingType::Storage { read_only },
//             has_dynamic_offset: false,
//             min_binding_size: None,
//         },
//         count: None,
//     }
// }

// fn layout_vertex(device: &Device) -> BindGroupLayout {
//     device.create_bind_group_layout(&BindGroupLayoutDescriptor {
//         label: Some(LABEL_VERTEX),
//         entries: &[layout_entry_vertex(0)],
//     })
// }

// fn layout_entry_vertex(binding: u32) -> BindGroupLayoutEntry {
//     BindGroupLayoutEntry {
//         binding,
//         visibility: ShaderStages::VERTEX,
//         ty: BindingType::Buffer {
//             ty: BufferBindingType::Storage { read_only: true },
//             has_dynamic_offset: false,
//             min_binding_size: None,
//         },
//         count: None,
//     }
// }
