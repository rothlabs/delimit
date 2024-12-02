use super::*;
use unit::*;
use uniform::*;

mod unit;
mod uniform;

struct Chunk {
    grant: Leaf<u32>,
    start: u32,
    end: u32,
}

#[derive(Debug, Clone)]
pub struct Store {
    pub device: Grc<Device>,
    uniform: Grc<UniformStore>,
    // storage: Section,
}

impl Store {
    // TODO: &Device
    pub fn new(gpu: &Gpu) -> Self {
        Self {
            device: gpu.device.clone(),
            uniform: Grc::new(UniformStore::new(gpu)),
        }
    }
    pub fn uniform(&self, size: Hub<u32>) -> Hub<u32> {
        Grant {
            size,
            kind: Kind::Uniform(self.uniform.clone()),
        }
        .hub()
    }
}

fn uniform_layout(device: &Device) -> BindGroupLayout {
    device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("gpu_store_unifrom_bind_group_layout"),
        entries: &[uniform_compute_entry(0)],
    })
}

fn uniform_compute_entry(binding: u32) -> BindGroupLayoutEntry {
    BindGroupLayoutEntry {
        binding,
        visibility: ShaderStages::COMPUTE,
        ty: BindingType::Buffer {
            ty: BufferBindingType::Uniform,
            has_dynamic_offset: true,
            min_binding_size: None,
        },
        count: None,
    }
}

fn storage_compute_entry(binding: u32) -> BindGroupLayoutEntry {
    BindGroupLayoutEntry {
        binding,
        visibility: ShaderStages::COMPUTE,
        ty: BindingType::Buffer {
            ty: BufferBindingType::Storage { read_only: false },
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

fn uniform_buffer(device: &Device) -> Grc<Buffer> {
    Grc::new(device.create_buffer(&BufferDescriptor {
        label: None,
        size: 1000,
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    }))
}

fn storage_buffer(device: &Device) -> Leaf<Grc<Buffer>> {
    Grc::new(device.create_buffer(&BufferDescriptor {
        label: None,
        size: 10000,
        usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    }))
    .into()
}
