use super::*;

#[derive(Debug)]
pub struct StorageStore {
    pub layout: BindGroupLayout,
    pub layout_vertex: BindGroupLayout,
    pub group: Leaf<Grc<BindGroup>>,
    pub group_vertex: Leaf<Grc<BindGroup>>,
    pub buffer: Leaf<Grc<Buffer>>,
    chunks: Leaf<Vec<Chunk>>,
}

impl StorageStore {
    pub fn new(device: &Device) -> Self {
        let buffer = storage_buffer(device);
        let layout = storage_layout(device);
        let layout_read = storage_layout_vertex(device);
        let entry = BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
        };
        let group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("gpu_store_storage"),
            layout: &layout,
            entries: &[entry.clone()],
        });
        let group_read = device.create_bind_group(&BindGroupDescriptor {
            label: Some("gpu_store_storage_read"),
            layout: &layout_read,
            entries: &[entry],
        });
        Self {
            layout,
            layout_vertex: layout_read,
            group: Leaf::new(Grc::new(group)),
            group_vertex: Leaf::new(Grc::new(group_read)),
            buffer: Leaf::new(buffer),
            chunks: Leaf::default(),
        }
    }
    pub fn grant(&self, size: u32) -> Result<Leaf<u32>> {
        let max = (self.buffer.base()?.size() / 4) as u32;
        // println!("storage");
        grant(&self.chunks, size, max)
    }
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