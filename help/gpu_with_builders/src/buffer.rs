pub use blank::*;
pub use reader::*;
pub use size::*;
use typed_builder::TypedBuilder;
pub use uniform::*;
pub use writer::*;

use super::*;

mod blank;
mod reader;
mod size;
mod uniform;
mod writer;

#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "graph::Error"))]
pub struct BufferRig<'a> {
    device: &'a Device,
    #[builder(default, setter(strip_option))]
    label: Option<&'a str>,
    size: u64,
    usage: BufferUsages,
    #[builder(default)]
    mapped_at_creation: bool,
}

impl BufferRigBuilder<'_> {
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
    pub fn storage(self) -> graph::Result<Grc<Buffer>> {
        self.usage(BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST)
            .make()
    }
    pub fn uniform(self) -> graph::Result<Grc<Buffer>> {
        self.usage(BufferUsages::UNIFORM | BufferUsages::COPY_DST)
            .make()
    }
    pub fn vertex(self) -> graph::Result<Grc<Buffer>> {
        self.usage(BufferUsages::VERTEX | BufferUsages::COPY_DST)
            .make()
    }
}

// pub fn map_read(self) -> graph::Result<Grc<Buffer>> {
//     self.usage(BufferUsages::MAP_READ | BufferUsages::COPY_DST)
//         .make()
// }

#[derive(TypedBuilder)]
pub struct BufferRig2<'a> {
    device: &'a Device,
    #[builder(default, setter(strip_option))]
    label: Option<&'a str>,
    size: u64,
    usage: BufferUsages,
    #[builder(default)]
    mapped_at_creation: bool,
}