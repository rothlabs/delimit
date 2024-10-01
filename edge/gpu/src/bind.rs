use super::*;

#[derive(Builder, Debug)]
#[builder(pattern = "owned", build_fn(error = "crate::Error"))]
pub struct Bind<'a> {
    device: &'a Device,
    #[builder(default)]
    label: Option<&'a str>,
    layout: &'a BindGroupLayout,
    #[builder(setter(each(name = "entry")))]
    entries: &'a [BindGroupEntry<'a>]
}

impl BindBuilder<'_> {
    pub fn make(self) -> Result<wgpu::BindGroup> {
        let built = self.build()?;
        let descriptor = BindGroupDescriptor {
            label: built.label,
            layout: built.layout,
            entries: built.entries,
        };
        let value = built.device.create_bind_group(&descriptor);
        Ok(value)
    }
}