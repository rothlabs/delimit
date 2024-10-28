use super::*;
use std::num::NonZero;

#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "graph::Error"))]
pub struct BindLayout<'a> {
    device: &'a Device,
    #[builder(default)]
    label: Option<&'a str>,
    #[builder(default)]
    entries: &'a [BindGroupLayoutEntry],
}

impl<'a> BindLayoutBuilder<'a> {
    pub fn make(self) -> graph::Result<Grc<BindGroupLayout>> {
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
#[builder(build_fn(error = "graph::Error"))]
#[builder(setter(strip_option))]
pub struct BindEntry {
    binding: u32,
    visibility: ShaderStages,
    ty: BindingType,
    #[builder(default)]
    count: Option<NonZero<u32>>,
}

impl BindEntryBuilder {
    pub fn make(self) -> graph::Result<BindGroupLayoutEntry> {
        let built = self.build()?;
        let out = BindGroupLayoutEntry {
            binding: built.binding,
            visibility: built.visibility,
            ty: built.ty,
            count: built.count,
        };
        Ok(out)
    }
    pub fn compute(self) -> graph::Result<BindGroupLayoutEntry> {
        self.visibility(ShaderStages::COMPUTE).make()
    }
    pub fn vertex(self) -> graph::Result<BindGroupLayoutEntry> {
        self.visibility(ShaderStages::VERTEX).make()
    }
}

#[derive(Builder)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "graph::Error"))]
#[builder(setter(strip_option))]
pub struct BufferBinding {
    ty: BufferBindingType,
    #[builder(default)]
    has_dynamic_offset: bool,
    #[builder(default)]
    min_binding_size: Option<NonZero<u64>>,
}

impl BufferBindingBuilder {
    pub fn make(self) -> graph::Result<BindingType> {
        let built = self.build()?;
        let out = BindingType::Buffer {
            ty: built.ty,
            has_dynamic_offset: built.has_dynamic_offset,
            min_binding_size: built.min_binding_size,
        };
        Ok(out)
    }
    pub fn entry(self, binding: u32) -> graph::Result<BindEntryBuilder> {
        let binding_type = self.make()?;
        let out = BindEntryBuilder::default()
            .binding(binding)
            .ty(binding_type);
        Ok(out)
    }
}
