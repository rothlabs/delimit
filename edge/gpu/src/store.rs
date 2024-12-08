use super::*;
use mesh::*;
use rig::*;
use topic::*;
use unit::*;

mod mesh;
mod rig;
mod topic;
mod unit;

#[derive(Debug)]
pub struct Store {
    // pub layout: PipelineLayout,
    pub rig: Grc<Rig>,
    pub topic: Grc<Topic>,

    // for vertex data of position only
    // need another mesh store to for other vertex formats
    pub mesh: Grc<Mesh>,
}

impl Store {
    pub fn new(device: &Device) -> Self {
        let rig = Rig::new(device);
        let topic = Topic::new(device);
        // let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        //     label: Some("gpu_store"),
        //     bind_group_layouts: &[&topic.layout, &rig.layout],
        //     push_constant_ranges: &[],
        // });
        Self {
            // layout,
            rig: Grc::new(rig),
            topic: Grc::new(topic),
            mesh: Grc::new(Mesh::new(device)),
        }
    }
    pub fn rig(&self) -> Hub<u32> {
        Grant {
            size: 64.into(),
            kind: Kind::Rig(self.rig.clone()),
        }
        .hub()
    }
    pub fn topic(&self, size: impl Into<Hub<u32>>) -> Hub<u32> {
        Grant {
            size: size.into(),
            kind: Kind::Topic(self.topic.clone()),
        }
        .hub()
    }
    pub fn mesh(&self, size: impl Into<Hub<u32>>) -> Hub<u32> {
        Grant {
            size: size.into(),
            kind: Kind::Mesh(self.mesh.clone()),
        }
        .hub()
    }
}

#[derive(Debug)]
pub enum Kind {
    Rig(Grc<Rig>),
    Topic(Grc<Topic>),
    Mesh(Grc<Mesh>),
}

struct Chunk {
    // grant: Leaf<u32>,
    start: u32,
    end: u32,
}

fn storage_buffer(device: &Device) -> Grc<Buffer> {
    Grc::new(device.create_buffer(&BufferDescriptor {
        label: None,
        ///// 1000000 bytes = 1 mb
        size: 1000000,
        usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    }))
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
            // TODO: make bigger buffer and copy to it
            panic!("buffer full!")
        }
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

// pub fn rig_bind(&self, slot: impl Into<Hub<u32>>) -> RigBind {
//     let offset = self.rig();
//     let bind = Bind {
//         slot: slot.into(),
//         group: self.rig.group.hub(),
//         offsets: vec![offset.clone()],
//     }
//     .hub();
//     RigBind {
//         bind,
//         index: offset,
//     }
// }

// pub struct RigBind {
//     pub bind: Hub<stable::GroupBind>,
//     pub index: Hub<u32>,
// }

// pub fn rig(&self) -> Hub<u32> {
//     Grant {
//         size: 64.into(),
//         kind: Kind::Rig(self.rig.clone()),
//     }
//     .hub()
// }

// fn uniform_buffer(device: &Device) -> Grc<Buffer> {
//     Grc::new(device.create_buffer(&BufferDescriptor {
//         label: None,
//         size: 65536,
//         usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST | BufferUsages::COPY_DST,
//         mapped_at_creation: false,
//     }))
// }
