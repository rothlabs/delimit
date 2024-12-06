use super::*;

pub struct GroupLayout<'a> {
    pub device: &'a Device,
    // pub store: &'a Store,
    pub rig: Grc<BindGroupLayout>,
    pub topic: Grc<BindGroupLayout>,
    pub image: Grc<BindGroupLayout>,
}

impl<'a> GroupLayout<'a> {
    pub fn new(gpu: &'a Gpu) -> Self {
        let device = &gpu.device;
        Self { device, rig: rig(device), topic: topic(device), image: image(device) }
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

pub fn rig(device: &Device) -> Grc<BindGroupLayout> {
    device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("mech_rig"),
        entries: &[rig_entry()],
    }).into()
}

fn rig_entry() -> BindGroupLayoutEntry {
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


////////////

// fn entry(buffer: &Grc<Buffer>) -> BindGroupEntry {
//     BindGroupEntry {
//         binding: 0,
//         resource: buffer.as_entire_binding(),
//     }
// }

pub fn topic(device: &Device) -> Grc<BindGroupLayout> {
    device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("mech_topic"),
        entries: &[topic_entry()],
    }).into()
}

fn topic_entry() -> BindGroupLayoutEntry {
    BindGroupLayoutEntry {
        binding: 0,
        visibility: ShaderStages::COMPUTE,
        ty: BindingType::Buffer {
            ty: BufferBindingType::Storage { read_only: false },
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

pub fn image(device: &Device) -> Grc<BindGroupLayout> {
    device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("mech_image_topic"),
        entries: &[image_entry()],
    }).into()
}

fn image_entry() -> BindGroupLayoutEntry {
    BindGroupLayoutEntry {
        binding: 0,
        visibility: ShaderStages::VERTEX,
        ty: BindingType::Buffer {
            ty: BufferBindingType::Storage { read_only: true },
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}