use super::*;
use unit::*;

mod unit;

#[derive(Debug)]
pub struct Block {
    pub buffer: Leaf<Grc<Buffer>>,
    chunks: Leaf<Vec<Chunk>>,
}

impl Block {
    pub fn new(device: &Device) -> Self {
        let buffer = storage_buffer(device);
        Self {
            buffer: Leaf::new(buffer),
            chunks: Leaf::default(),
        }
    }
    pub fn grant(&self, size: u32) -> Result<Leaf<u32>> {
        let max = (self.buffer.base()?.size() / 4) as u32;
        grant(&self.chunks, size, max)
    }
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


// pub fn mesh(&self, size: impl Into<Hub<u32>>) -> Hub<u32> {
//     Grant {
//         size: size.into(),
//         kind: Kind::Mesh(self.mesh.clone()),
//     }
//     .hub()
// }

    // for vertex data of position only
    // need another mesh store to for other vertex formats
    // pub mesh: Grc<Mesh>,

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
