use super::*;

#[derive(Builder, Debug)]
#[builder(pattern = "owned")] 
#[builder(build_fn(error = "crate::Error"))]
pub struct BindGroup<'a> {
    device: &'a Device,
    #[builder(default)]
    label: Option<&'a str>,
    layout: &'a wgpu::BindGroupLayout,
    // #[builder(setter(each(name = "entry")))]
    entries: Vec<BindGroupEntry<'a>>
}

impl<'a> BindGroupBuilder<'a> {
    pub fn make(self) -> Result<wgpu::BindGroup> {
        let built = self.build()?;
        let descriptor = BindGroupDescriptor {
            label: built.label,
            layout: built.layout,
            entries: &built.entries,
        };
        let value = built.device.create_bind_group(&descriptor);
        Ok(value)
    }
    pub fn entry(mut self, binding: u32, resource: BindingResource<'a>) -> Self {
        if let Some(mut entries) = self.entries {
            entries.push(BindGroupEntry {
                binding,
                resource
            });
            self.entries = Some(entries);
        } else {
            self.entries = Some(vec![BindGroupEntry {
                binding,
                resource
            }]);
        }
        self
    }
}

#[derive(Builder, Debug)]
#[builder(pattern = "owned")] 
#[builder(build_fn(error = "crate::Error"))]
pub struct BindGroupLayout<'a> {
    device: &'a Device,
    #[builder(default)]
    label: Option<&'a str>,
    #[builder(default, setter(each(name = "entry")))]
    entries: &'a [BindGroupLayoutEntry]
}

impl BindGroupLayoutBuilder<'_> {
    pub fn make(self) -> Result<wgpu::BindGroupLayout> {
        let built = self.build()?;
        let descriptor = BindGroupLayoutDescriptor {
            label: built.label,
            entries: built.entries,
        };
        let value = built.device.create_bind_group_layout(&descriptor);
        Ok(value)
    }
}