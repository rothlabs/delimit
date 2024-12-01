use super::*;

pub fn store(gpu: &Gpu) -> Store {
    Store {
        device: gpu.device.clone(),
        uniform: Section::new(uniform_buffer(&gpu.device)),
        storage: Section::new(storage_buffer(&gpu.device)),
    }
}

// #[derive(Debug, Clone)]
pub struct Store {
    pub device: Grc<Device>,
    uniform: Section,
    storage: Section,
}

impl Store {
    pub fn uniform(&self, length: u32) -> Result<Grant> {
        let mut i = 0;
        let mut start = 0;
        let mut end = length;
        let offset = self.uniform.grants.write_passive(|grants| {
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
    fn allocate(&self, allocation: &mut Section, chunk: Chunck, i: usize) -> Grc<u32> {
        let offset = Grc::new(chunk.start);
        allocation.chuncks.insert(i, chunk);
        allocation.grants.insert(i, offset.clone());
        offset
    }
}

// #[derive(Default)]
pub struct Section {
    buffer: Leaf<Grc<Buffer>>,
    // chuncks: Vec<Chunck>,
    grants: Leaf<Vec<Grc<Grant>>>,
}

// struct Layout {
//     chuncks: Vec<Chunck>,
//     grants: Vec<Grc<Grant>>,
// }

impl Section {
    fn new(buffer: Leaf<Grc<Buffer>>) -> Self {
        Self {
            buffer,
            // chuncks: vec![],
            grants: vec![],
        }
    }
}

// #[derive(Debug, Clone)]
pub struct Chunck {
    start: u32,
    end: u32,
}

pub struct Grant {
    buffer: Hub<Grc<Buffer>>,
    offset: Hub<u32>,
}

fn uniform_buffer(device: &Device) -> Leaf<Grc<Buffer>> {
    Grc::new(device.create_buffer(&BufferDescriptor {
        label: None,
        size: 1000,
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })).into()
}

fn storage_buffer(device: &Device) -> Leaf<Grc<Buffer>> {
    Grc::new(device.create_buffer(&BufferDescriptor {
        label: None,
        size: 10000,
        usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })).into()
}