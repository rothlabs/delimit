pub use reader::*;
pub use sizer::*;
pub use uniform::*;
pub use writer::*;

use super::*;

mod reader;
mod sizer;
mod uniform;
mod writer;

#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "graph::Error"))]
pub struct BufferSetup<'a> {
    device: &'a Device,
    #[builder(default, setter(strip_option))]
    label: Option<&'a str>,
    size: u64,
    usage: BufferUsages,
    #[builder(default)]
    mapped_at_creation: bool,
}

impl BufferSetupBuilder<'_> {
    pub fn make(self) -> graph::Result<Grc<Buffer>> {
        let built = self.build()?;
        let descriptor = BufferDescriptor {
            label: built.label,
            size: built.size,
            usage: built.usage,
            mapped_at_creation: built.mapped_at_creation,
        };
        let buffer = built.device.create_buffer(&descriptor);
        Ok(buffer.into())
    }
    pub fn map_read(self) -> graph::Result<Grc<Buffer>> {
        self.usage(BufferUsages::MAP_READ | BufferUsages::COPY_DST)
            .make()
    }
    pub fn storage_copy(self) -> graph::Result<Grc<Buffer>> {
        self.usage(BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST)
            .make()
    }
}