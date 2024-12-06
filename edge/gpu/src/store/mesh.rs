use super::*;

const LABEL: &str = "gpu_store_model";

#[derive(Debug)]
pub struct Mesh {
    pub vertex: Hub<action::BufferBind>,
    pub buffer: Leaf<Grc<Buffer>>,
    chunks: Leaf<Vec<Chunk>>,
}

impl Mesh {
    pub fn new(device: &Device) -> Self {
        let buffer = vertex_buffer(device);
        let vertex = Vertex::builder().buffer(&buffer).build().hub();
        // let vertex = Vertex {
        //     slot: 0.into(),
        //     buffer: buffer.clone().into(),
        //     offset: None,
        // }
        // .hub();
        Self {
            vertex,
            buffer: Leaf::new(buffer),
            chunks: Leaf::default(),
        }
    }
    pub fn grant(&self, size: u32) -> Result<Leaf<u32>> {
        let max = (self.buffer.base()?.size() / 4) as u32;
        grant(&self.chunks, size, max)
    }
}

fn vertex_buffer(device: &Device) -> Grc<Buffer> {
    Grc::new(device.create_buffer(&BufferDescriptor {
        label: Some(LABEL),
        size: 100000,
        usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    }))
}
