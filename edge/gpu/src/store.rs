use super::*;
use topic::*;
use rig::*;
use unit::*;

mod topic;
mod rig;
mod unit;

#[derive(Debug, Clone)]
pub struct Store {
    pub rig: Grc<Rig>,
    pub topic: Grc<Topic>,
}

impl Store {
    pub fn new(device: &Device) -> Self {
        Self {
            rig: Grc::new(Rig::new(device)),
            topic: Grc::new(Topic::new(device)),
        }
    }
    pub fn rig(&self, size: impl Into<Hub<u32>>) -> Hub<u32> {
        Grant {
            size: size.into(),
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
}

fn storage_buffer(device: &Device) -> Grc<Buffer> {
    Grc::new(device.create_buffer(&BufferDescriptor {
        label: None,
        // 100000 bytes = 0.1 mb
        size: 100000,
        usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    }))
}

#[derive(Debug)]
pub enum Kind {
    Rig(Grc<Rig>),
    Topic(Grc<Topic>),
}

struct Chunk {
    // grant: Leaf<u32>,
    start: u32,
    end: u32,
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

// fn uniform_buffer(device: &Device) -> Grc<Buffer> {
//     Grc::new(device.create_buffer(&BufferDescriptor {
//         label: None,
//         size: 65536,
//         usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST | BufferUsages::COPY_DST,
//         mapped_at_creation: false,
//     }))
// }