use super::*;

pub fn store(gpu: &Gpu) -> Store {
    Store {
        device: gpu.device.clone(),
        uniform: Section::new(uniform_buffer(&gpu.device)),
        storage: Section::new(storage_buffer(&gpu.device)),
    }
}

#[derive(Debug, Clone)]
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
        let offset = self.uniform.chunks.write_passive(|chunks| {
            while let Some(chunk) = chunks.get(i) {
                if end < *chunk.start {
                    break;
                } else {
                    start = chunk.end;
                    end = chunk.end + length;
                }
                i += 1;
            }
            let offset = Grc::new(start);
            let chunk = Chunk { start: offset.clone(), end };
            chunks.insert(i, chunk);
            Grant {
                buffer: self.uniform.buffer.clone().hub(),
                offset: offset.into()
            }
        })?;
        Ok(offset)
    }
}

#[derive(Debug, Clone)]
pub struct Section {
    buffer: Leaf<Grc<Buffer>>,
    chunks: Leaf<Vec<Chunk>>,
}

impl Section {
    fn new(buffer: Leaf<Grc<Buffer>>) -> Self {
        Self {
            buffer,
            chunks: Leaf::default(),
        }
    }
}

struct Chunk {
    start: Grc<u32>,
    end: u32,
}

pub struct Grant {
    buffer: Hub<Grc<Buffer>>,
    offset: Hub<Grc<u32>>,
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