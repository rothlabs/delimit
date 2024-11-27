use super::*;

pub struct Store {
    gpu: Core,
    storage: Grc<Buffer>,
    allocation: Leaf<Allocation>,
}

impl Store {
    pub fn new(gpu: Core) -> Self {
        let storage = gpu.buffer(100000).storage().unwrap();
        Self {
            gpu,
            storage,
            allocation: Leaf::default(),
        }
    }
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
