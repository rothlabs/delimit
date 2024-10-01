use super::*;

#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "crate::Error"))]
pub struct Buffer<'a> {
    device: &'a Device,
    #[builder(default)]
    label: Option<&'a str>,
    size: u64,
    usage: BufferUsages,
    #[builder(default)]
    mapped_at_creation: bool,
}

impl BufferBuilder<'_> {
    pub fn make(self) -> Result<Grc<wgpu::Buffer>> {
        let built = self.build()?;
        let descriptor = BufferDescriptor {
            label: built.label,
            size: built.size,
            usage: built.usage,
            mapped_at_creation: built.mapped_at_creation,
        };
        let value = built.device.create_buffer(&descriptor);
        Ok(value.into())
    }
}


// #[derive(Builder, Debug)]
// #[builder(pattern = "owned", build_fn(error = "crate::Error"))]
// pub struct Buffer<'a> {
//     device: &'a Device,
//     label: Option<&'a str>,
//     size: u64,
//     usage: BufferUsages,
//     mapped_at_creation: bool,
// }

// impl BufferBuilder<'_> {
//     pub fn make(self) -> Result<wgpu::Buffer> {
//         let built = self.build()?;
//         let descriptor = BufferDescriptor {
//             label: built.label,
//             size: built.size,
//             usage: built.usage,
//             mapped_at_creation: built.mapped_at_creation,
//         };
//         let value = built.device.create_buffer(&descriptor);
//         Ok(value)
//     }
// }