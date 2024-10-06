use super::*;
use std::ops::Deref;

#[derive(Clone)]
pub struct Shader<'a> {
    pub inner: Grc<ShaderModule>,
    pub surface: &'a crate::Surface<'a>,
    // pub device: Grc<Device>,
}

impl<'a> Shader<'a> {
    pub fn vertex(&'a self, entry: &'a str) -> VertexBuilder<'a> {
        VertexBuilder::default().shader(self).entry(entry)
    }
    pub fn fragment(&'a self, entry: &'a str) -> FragmentBuilder<'a> {
        FragmentBuilder::default().shader(self).entry(entry).local_surface(self.surface)
    }
}

impl Deref for Shader<'_> {
    type Target = ShaderModule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}