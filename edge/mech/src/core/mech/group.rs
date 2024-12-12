use super::*;

#[derive(Debug, Clone)]
pub struct Bind {
    pub topic: Hub<stable::GroupBind>,
    pub image: Hub<stable::GroupBind>,
}

impl Bind {
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

pub struct Layout<'a> {
    pub device: &'a Grc<Device>,
    pub store: &'a Store,
    pub rig: Grc<BindGroupLayout>,
    pub topic: Grc<BindGroupLayout>,
    pub image: Grc<BindGroupLayout>,
}

impl<'a> Layout<'a> {
    pub fn new(device: &'a Grc<Device>, store: &'a Store) -> Self {
        // let device = &gpu.device;
        // let store = &gpu.store;
        Self {
            device,
            store,
            rig: rig(device),
            topic: topic(device),
            image: image(device),
        }
    }
}

pub fn rig(device: &Device) -> Grc<BindGroupLayout> {
    device
        .create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("mech_rig"),
            entries: &[rig_entry()],
        })
        .into()
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

pub fn topic(device: &Device) -> Grc<BindGroupLayout> {
    device
        .create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("mech_topic"),
            entries: &[topic_entry()],
        })
        .into()
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
    device
        .create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("mech_image_topic"),
            entries: &[image_entry()],
        })
        .into()
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
