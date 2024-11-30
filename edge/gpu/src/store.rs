use super::*;

pub fn store(gpu: &Gpu) -> Store {
    let uniform = Grc::new(gpu.device.create_buffer(&uniform_rig()));
    let storage0 = Grc::new(gpu.device.create_buffer(&storage_rig()));
    let storage1 = Grc::new(gpu.device.create_buffer(&storage_rig()));
    Store {
        device: gpu.device.clone(),
        uniform: Leaf::new(uniform),
        storage0: Leaf::new(storage0),
        storage1: Leaf::new(storage1),
        allocation: Leaf::default(),
    }
}

fn uniform_rig<'a>() -> BufferDescriptor<'a> {
    BufferDescriptor {
        label: None,
        size: 1000,
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    }
}

fn storage_rig<'a>() -> BufferDescriptor<'a> {
    BufferDescriptor {
        label: None,
        size: 10000,
        usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    }
}

#[derive(Debug, Clone)]
pub struct Store {
    pub device: Grc<Device>,
    pub uniform: Leaf<Grc<Buffer>>,
    pub storage0: Leaf<Grc<Buffer>>,
    pub storage1: Leaf<Grc<Buffer>>,
    allocation: Leaf<Allocation>,
}

impl Store {
    pub fn storage(&self, length: u32) -> Result<Grc<u32>> {
        let mut i = 0;
        let mut start = 0;
        let mut end = length;
        let offset = self.allocation.write_passive(|allocation| {
            while let Some(chunk) = allocation.chuncks.get(i) {
                if end < chunk.start {
                    break;
                } else {
                    start = chunk.end;
                    end = start + length;
                }
                i += 1;
            }
            self.allocate(allocation, Chunck { start, end }, i)
        })?;
        Ok(offset)
    }
    fn allocate(&self, allocation: &mut Allocation, chunk: Chunck, i: usize) -> Grc<u32> {
        let offset = Grc::new(chunk.start);
        allocation.chuncks.insert(i, chunk);
        allocation.offsets.insert(i, offset.clone());
        offset
    }
}

#[derive(Default)]
pub struct Allocation {
    chuncks: Vec<Chunck>,
    offsets: Vec<Grc<u32>>,
}

// #[derive(Debug, Clone)]
pub struct Chunck {
    start: u32,
    end: u32,
}
