use super::*;

pub struct BindBank {
    storage: Grc<BindGroup>,
    // entries: Vec<BindGroupLayoutEntry>,
}

impl BindBank {
    pub fn new(store: &Store) {
        let rig = BindRig::default();
        let layout = store.device.create_bind_group_layout(&rig.layout());
        // let wow = store.device.create_bind_group(layout);
        0
    }
}

pub struct BindRig {
    layout_entries: Vec<BindGroupLayoutEntry>,
}

impl Default for BindRig {
    fn default() -> Self {
        Self {
            layout_entries: vec![compute_storage_0()]
        }
    }
}

impl BindRig {
    fn layout(&self) -> BindGroupLayoutDescriptor {
        BindGroupLayoutDescriptor {
            label: None,
            entries: &self.layout_entries,
        }
    }
    // fn bind_group_rig(&self, device: &Device) -> BindGroupDescriptor {
    //     let layout = &device.create_bind_group_layout(&self.layout());
    //     BindGroupDescriptor {
    //         label: None,
    //         layout,
    //         entries: &entries,
    //     }
    // }
}

fn compute_storage_0() -> BindGroupLayoutEntry {
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

// pub struct PipeRig {
//     entries: Vec<BindGroupLayout>,
// }

// impl Default for PipeRig {
//     fn default() -> Self {
//         Self {
//             entries: vec![],
//         }
//     }
// }