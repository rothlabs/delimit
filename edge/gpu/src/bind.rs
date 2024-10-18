use std::num::NonZero;

use super::*;

#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "graph::Error"))]
#[builder(setter(strip_option))]
pub struct Bind<'a> {
    device: &'a Device,
    #[builder(default)]
    label: Option<&'a str>,
    #[builder(default)]
    layout: Option<&'a BindGroupLayout>,
    #[builder(default)]
    entries: Vec<BindGroupEntry<'a>>,
    #[builder(default)]
    pipe: Option<&'a ComputePipeline>,
}

impl Bind<'_> {
    fn make(self, layout: &BindGroupLayout) -> Grc<BindGroup> {
        let descriptor = BindGroupDescriptor {
            label: self.label,
            layout,
            entries: &self.entries,
        };
        self.device.create_bind_group(&descriptor).into()
    }
}

impl<'a> BindBuilder<'a> {
    pub fn make(self) -> graph::Result<Grc<BindGroup>> {
        let built = self.build()?;
        if let Some(layout) = built.layout {
            Ok(built.make(layout))
        } else if let Some(pipe) = built.pipe {
            Ok(built.make(&pipe.get_bind_group_layout(0)))
        } else {
            Err(anyhow!("no layout for bind group"))?
        }
    }
    pub fn entry(mut self, binding: u32, buffer: &'a Buffer) -> Self {
        let resource = buffer.as_entire_binding();
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
    #[builder(default)]
    entries: &'a [BindGroupLayoutEntry],
}

impl<'a> BindLayoutBuilder<'a> {
    pub fn make(self) -> Result<Grc<BindGroupLayout>> {
        let built = self.build()?;
        let descriptor = BindGroupLayoutDescriptor {
            label: built.label,
            entries: built.entries,
        };
        let out = built.device.create_bind_group_layout(&descriptor);
        Ok(out.into())
    }
}

#[derive(Builder)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "crate::Error"))]
#[builder(setter(strip_option))]
pub struct BindEntry {
    binding: u32,
    visibility: ShaderStages,
    ty: BindingType,
    #[builder(default)]
    count: Option<NonZero<u32>>,
}

impl BindEntryBuilder {
    pub fn make(self) -> Result<BindGroupLayoutEntry> {
        let built = self.build()?;
        let out = BindGroupLayoutEntry {
            binding: built.binding,
            visibility: built.visibility,
            ty: built.ty,
            count: built.count,
        };
        Ok(out)
    }
    pub fn compute(self) -> Result<BindGroupLayoutEntry> {
        self.visibility(ShaderStages::COMPUTE).make()
    }
}

#[derive(Builder)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "crate::Error"))]
#[builder(setter(strip_option))]
pub struct BufferBinding {
    ty: BufferBindingType,
    #[builder(default)]
    has_dynamic_offset: bool,
    #[builder(default)]
    min_binding_size: Option<NonZero<u64>>,
}

impl BufferBindingBuilder {
    pub fn make(self) -> Result<BindingType> {
        let built = self.build()?;
        let out = BindingType::Buffer {
            ty: built.ty,
            has_dynamic_offset: built.has_dynamic_offset,
            min_binding_size: built.min_binding_size,
        };
        Ok(out)
    }
    pub fn entry(self, binding: u32) -> Result<BindEntryBuilder> {
        let binding_type = self.make()?;
        let out = BindEntryBuilder::default()
            .binding(binding)
            .ty(binding_type);
        Ok(out)
    }
}
