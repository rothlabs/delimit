use super::*;
use storage::*;
use rig::*;
use unit::*;

mod storage;
mod rig;
mod unit;

struct Chunk {
    // grant: Leaf<u32>,
    start: u32,
    end: u32,
}

#[derive(Debug, Clone)]
pub struct Store {
    // pub device: Grc<Device>,
    pub rig: Grc<RigStore>,
    pub storage: Grc<StorageStore>,
}

impl Store {
    // TODO: &Device
    pub fn new(device: &Device) -> Self {
        Self {
            // device: gpu.device.clone(),
            rig: Grc::new(RigStore::new(device)),
            storage: Grc::new(StorageStore::new(device)),
        }
    }
    pub fn rig(&self, size: impl Into<Hub<u32>>) -> Hub<u32> {
        Grant {
            size: size.into(),
            kind: Kind::Rig(self.rig.clone()),
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

fn storage_buffer(device: &Device) -> Grc<Buffer> {
    Grc::new(device.create_buffer(&BufferDescriptor {
        label: None,
        // 1000000 bytes = 1 mb
        size: 1000000,
        usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    }))
}

// fn uniform_buffer(device: &Device) -> Grc<Buffer> {
//     Grc::new(device.create_buffer(&BufferDescriptor {
//         label: None,
//         size: 65536,
//         usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST | BufferUsages::COPY_DST,
//         mapped_at_creation: false,
//     }))
// }