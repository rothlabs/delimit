use super::*;
use storage::*;
use uniform::*;
use unit::*;

mod storage;
mod uniform;
mod unit;

struct Chunk {
    // grant: Leaf<u32>,
    start: u32,
    end: u32,
}

#[derive(Debug, Clone)]
pub struct Store {
    // pub device: Grc<Device>,
    pub uniform: Grc<UniformStore>,
    pub storage: Grc<StorageStore>,
}

impl Store {
    // TODO: &Device
    pub fn new(device: &Device) -> Self {
        Self {
            // device: gpu.device.clone(),
            uniform: Grc::new(UniformStore::new(device)),
            storage: Grc::new(StorageStore::new(device)),
        }
    }
    pub fn uniform(&self, size: impl Into<Hub<u32>>) -> Hub<u32> {
        Grant {
            size: size.into(),
            kind: Kind::Uniform(self.uniform.clone()),
        }
        .hub()
    }
    pub fn storage(&self, size: impl Into<Hub<u32>>) -> Hub<u32> {
        Grant {
            size: size.into(),
            kind: Kind::Storage(self.storage.clone()),
        }
        .hub()
    }
}

fn grant(chunks: &Leaf<Vec<Chunk>>, size: u32, max: u32) -> Result<Leaf<u32>> {
    let mut i = 0;
    let mut start = 0;
    let mut end = size;
    let grant = chunks.write_passive(|chunks| {
        while let Some(chunk) = chunks.get(i) {
            if end < chunk.start {
                break;
            } else {
                start = chunk.end;
                end = chunk.end + size;
            }
            i += 1;
        }
        if end > max {
            panic!("buffer full!")
        }
        // println!("buffer offset in elements: {start}");
        let grant = Leaf::new(start);
        let chunk = Chunk {
            // grant: grant.clone(),
            start,
            end,
        };
        chunks.insert(i, chunk);
        grant
    })?;
    Ok(grant)
}

fn uniform_layout(device: &Device) -> BindGroupLayout {
    device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("gpu_store_unifrom"),
        entries: &[uniform_compute_entry(0)],
    })
}

fn uniform_layout_vertex(device: &Device) -> BindGroupLayout {
    device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("gpu_store_unifrom"),
        entries: &[uniform_vertex_entry(0)],
    })
}

fn storage_layout(device: &Device) -> BindGroupLayout {
    device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("gpu_store_storage"),
        entries: &[
            storage_compute_entry(0, false),
            // storage_compute_entry(1, true),
        ],
    })
}

fn storage_layout_vertex(device: &Device) -> BindGroupLayout {
    device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("gpu_store_storage_read"),
        entries: &[storage_vertex_entry(0)],
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

fn uniform_vertex_entry(binding: u32) -> BindGroupLayoutEntry {
    BindGroupLayoutEntry {
        binding,
        visibility: ShaderStages::VERTEX,
        ty: BindingType::Buffer {
            ty: BufferBindingType::Uniform,
            has_dynamic_offset: true,
            min_binding_size: None,
        },
        count: None,
    }
}

fn storage_compute_entry(binding: u32, read_only: bool) -> BindGroupLayoutEntry {
    BindGroupLayoutEntry {
        binding,
        visibility: ShaderStages::COMPUTE,
        ty: BindingType::Buffer {
            ty: BufferBindingType::Storage { read_only },
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

fn storage_vertex_entry(binding: u32) -> BindGroupLayoutEntry {
    BindGroupLayoutEntry {
        binding,
        visibility: ShaderStages::VERTEX,
        ty: BindingType::Buffer {
            ty: BufferBindingType::Storage { read_only: true },
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

fn uniform_buffer(device: &Device) -> Grc<Buffer> {
    Grc::new(device.create_buffer(&BufferDescriptor {
        label: None,
        size: 65536,
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    }))
}

fn storage_buffer(device: &Device) -> Grc<Buffer> {
    Grc::new(device.create_buffer(&BufferDescriptor {
        label: None,
        // 1000000 bytes = 1 mb
        size: 1000000,
        usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    }))
}
