use super::*;

#[derive(Debug)]
pub struct Bank {
    // uniform: Hub<Grc<BindGroup>>,
    // storage: Hub<Grc<BindGroup>>,
}

impl Bank {
    pub fn new(store: &Store) -> Self {
        // let uniform = uniform_group(store);
        // let storage = storage_group(store);
        Self {  }
    }
}

// fn storage_group(store: &Store) -> Hub<Grc<BindGroup>> {
//     let device = store.device.clone();
//     BindGroupUnit {
//         layout: storage_layout(&device),
//         device,
//         buffers: vec![store.storage0.hub(), store.storage1.hub()],
//     }
//     .hub()
// }

fn storage_layout(device: &Device) -> BindGroupLayout {
    device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("mech_storage_bind_group_layout"),
        entries: &[storage_compute_entry(0), storage_compute_entry(1)],
    })
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

// fn uniform_group(store: &Store) -> Hub<Grc<BindGroup>> {
//     let device = store.device.clone();
//     BindGroupUnit {
//         layout: uniform_layout(&device),
//         device,
//         buffers: vec![store.uniform.hub()],
//     }
//     .hub()
// }

fn uniform_layout(device: &Device) -> BindGroupLayout {
    device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("mech_uniform_bind_group_layout"),
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





// fn bind_group(device: &Device) -> BindGroup {
//     device.create_bind_group(&BindGroupDescriptor{
//         label: None,
//         layout: &layout(device),
//         entries: &[],
//     })
// }

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

// #[derive(Default)]
// pub struct BindRig<'a> {
//     layout_entries: Vec<BindGroupLayoutEntry>,
//     entries: Vec<BindGroupEntry<'a>>,
// }

// // impl Default for BindRig {
// //     fn default() -> Self {
// //         Self {
// //             layout_entries: vec![compute_storage_0()]
// //         }
// //     }
// // }

// impl<'a> BindRig<'a> {
//     fn run(&mut self, device: &Device) {
//         // self.layout_entries = vec![compute_storage_0()];
//         // let layout_entries = vec![compute_storage_0()];
//         let rig = BindGroupLayoutDescriptor {
//             label: None,
//             entries: &[compute_storage_0()],
//         };
//         let layout = device.create_bind_group_layout(&rig);
//     }
//     fn layout(&self) -> BindGroupLayoutDescriptor {
//         BindGroupLayoutDescriptor {
//             label: None,
//             entries: &self.layout_entries,
//         }
//     }
//     // fn bind_group_rig(&self, device: &Device) -> BindGroupDescriptor {
//     //     let layout = &device.create_bind_group_layout(&self.layout());
//     //     BindGroupDescriptor {
//     //         label: None,
//     //         layout,
//     //         entries: &entries,
//     //     }
//     // }
// }
