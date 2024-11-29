use super::*;

pub struct BindBank {
    storage: Grc<BindGroup>,
    // entries: Vec<BindGroupLayoutEntry>,
}

impl BindBank {
    pub fn new(store: &Store) -> Self {
        // let layout = layout(&store.device);
        // let entry = BindGroupEntry {
        //     binding: 0,
        //     resource: store.storage,
        // };
        let storage = Grc::new(bind_group(&store.device));
        Self { storage }
    }
}

fn bind_group(device: &Device) -> BindGroup {
    device.create_bind_group(&BindGroupDescriptor{
        label: None,
        layout: &layout(device),
        entries: &[],
    })
}

fn layout(device: &Device) -> BindGroupLayout {
    device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: None,
        entries: &[compute_storage_0()],
    })
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