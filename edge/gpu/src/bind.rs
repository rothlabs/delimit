use std::num::NonZero;

use super::*;

#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "crate::Error"))]
#[builder(setter(strip_option))]
pub struct Bind<'a> {
    device: &'a Device,
    #[builder(default)]
    label: Option<&'a str>,
    #[builder(default)]
    layout: Option<&'a wgpu::BindGroupLayout>,
    #[builder(default)]
    entries: Vec<BindGroupEntry<'a>>,
    #[builder(default)]
    pipe: Option<&'a ComputePipeline>,
}

impl Bind<'_> {
    fn make(self, layout: &wgpu::BindGroupLayout) -> wgpu::BindGroup {
        let descriptor = BindGroupDescriptor {
            label: self.label,
            layout,
            entries: &self.entries,
        };
        self.device.create_bind_group(&descriptor)
    }
}

impl<'a> BindBuilder<'a> {
    pub fn make(self) -> Result<wgpu::BindGroup> {
        let built = self.build()?;
        if let Some(layout) = built.layout {
            Ok(built.make(layout))
        } else if let Some(pipe) = built.pipe {
            Ok(built.make(&pipe.get_bind_group_layout(0)))
        } else {
            Err(anyhow!("no layout for bind group"))?
        }
    }
    pub fn entry(mut self, binding: u32, buffer: &'a crate::Buffer) -> Self {
        let resource = buffer.resource();
        if let Some(mut entries) = self.entries {
            entries.push(BindGroupEntry { binding, resource });
            self.entries = Some(entries);
        } else {
            self.entries = Some(vec![BindGroupEntry { binding, resource }]);
        }
        self
    }
}

#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "crate::Error"))]
pub struct BindLayout<'a> {
    device: &'a Device,
    #[builder(default)]
    label: Option<&'a str>,
    #[builder(default)] // , setter(each(name = "entry")
    entries: &'a [BindGroupLayoutEntry],
}

impl<'a> BindLayoutBuilder<'a> {
    pub fn make(self) -> Result<wgpu::BindGroupLayout> {
        let built = self.build()?;
        let descriptor = BindGroupLayoutDescriptor {
            label: built.label,
            entries: built.entries,
        };
        let out = built.device.create_bind_group_layout(&descriptor);
        Ok(out)
    }
    pub fn entry(self) -> BindLayoutEntryBuilder<'a> {
        BindLayoutEntryBuilder::default().upper(self)
    }
}

#[derive(Builder)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "crate::Error"))]
#[builder(setter(strip_option))]
pub struct BindLayoutEntry<'a> {
    #[builder(default)]
    upper: Option<BindLayoutBuilder<'a>>,
    binding: u32,
    visibility: ShaderStages,
    ty: BindingType,
    #[builder(default)]
    count: Option<NonZero<u32>>,
}

impl<'a> BindLayoutEntryBuilder<'a> {
    pub fn make(self) -> Result<wgpu::BindGroupLayoutEntry> {
        let built = self.build()?;
        let out = wgpu::BindGroupLayoutEntry {
            binding: built.binding,
            visibility: built.visibility,
            ty: built.ty,
            count: built.count,
        };
        Ok(out)
    }
    pub fn buffer(self) -> BufferBindingBuilder<'a> {
        BufferBindingBuilder::default().upper(self)
    }
}

#[derive(Builder)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "crate::Error"))]
#[builder(setter(strip_option))]
pub struct BufferBinding<'a> {
    #[builder(default)]
    upper: Option<BindLayoutEntryBuilder<'a>>,
    ty: BufferBindingType,
    #[builder(default)]
    has_dynamic_offset: bool,
    #[builder(default)]
    min_binding_size: Option<NonZero<u64>>,
}

impl<'a> BufferBindingBuilder<'a> {
    pub fn make(self) -> Result<wgpu::BindingType> {
        let built = self.build()?;
        let out = wgpu::BindingType::Buffer {
            ty: built.ty,
            has_dynamic_offset: built.has_dynamic_offset,
            min_binding_size: built.min_binding_size,
        };
        Ok(out)
    }
}
