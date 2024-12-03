use std::num::NonZero;

use super::*;

#[derive(Debug)]
pub struct RigStore {
    pub layout: BindGroupLayout,
    // pub layout_vertex: BindGroupLayout,
    pub group: Leaf<Grc<BindGroup>>,
    // pub group_vertex: Leaf<Grc<BindGroup>>,
    pub buffer: Leaf<Grc<Buffer>>,
    chunks: Leaf<Vec<Chunk>>,
}

impl RigStore {
    pub fn new(device: &Device) -> Self {
        let buffer = storage_buffer(device);
        let layout = rig_layout(device);
        // let layout_vertex = uniform_layout_vertex(device);
        let entry = BindGroupEntry {
            binding: 0,
            resource: BindingResource::Buffer(BufferBinding {
                buffer: &buffer,
                offset: 0,
                size: Some(NonZero::new(256).unwrap()),
            }),
        };
        let group = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &layout,
            entries: &[entry.clone()],
        });
        // let group_vertex = device.create_bind_group(&BindGroupDescriptor {
        //     label: None,
        //     layout: &layout_vertex,
        //     entries: &[entry],
        // });
        Self {
            layout,
            // layout_vertex,
            group: Leaf::new(Grc::new(group)),
            // group_vertex: Leaf::new(Grc::new(group_vertex)),
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

fn rig_layout(device: &Device) -> BindGroupLayout {
    device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("gpu_store_unifrom"),
        entries: &[entry()],
    })
}

fn entry() -> BindGroupLayoutEntry {
    BindGroupLayoutEntry {
        binding: 0,
        visibility: ShaderStages::COMPUTE | ShaderStages::VERTEX,
        ty: BindingType::Buffer {
            ty: BufferBindingType::Storage { read_only: true },
            has_dynamic_offset: true,
            min_binding_size: None,
        },
        count: None,
    }
}

// fn rig_layout_vertex(device: &Device) -> BindGroupLayout {
//     device.create_bind_group_layout(&BindGroupLayoutDescriptor {
//         label: Some("gpu_store_unifrom"),
//         entries: &[entry_vertex()],
//     })
// }

// fn entry_vertex() -> BindGroupLayoutEntry {
//     BindGroupLayoutEntry {
//         binding: 0,
//         visibility: ShaderStages::VERTEX,
//         ty: BindingType::Buffer {
//             ty: BufferBindingType::Storage { read_only: true },
//             has_dynamic_offset: true,
//             min_binding_size: None,
//         },
//         count: None,
//     }
// }