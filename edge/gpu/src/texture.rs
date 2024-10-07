use super::*;

#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "crate::Error"))]
pub struct Texture<'a> {
    device: &'a Device,
    #[builder(default)]
    label: Option<&'a str>,
    size: Extent3d,
    mip_level_count: u32,
    #[builder(default = "1")]
    sample_count: u32,
    #[builder(default = "wgpu::TextureDimension::D2")]
    dimension: TextureDimension,
    format: TextureFormat,
    usage: TextureUsages,
    #[builder(default)]
    view_formats: &'a [TextureFormat],
}

impl<'a> TextureBuilder<'a> {
    pub fn make(self) -> Result<wgpu::Texture> {
        let built = self.build()?;
        let descriptor = TextureDescriptor {
            label: built.label,
            size: built.size,
            mip_level_count: built.mip_level_count,
            sample_count: built.sample_count,
            dimension: built.dimension,
            format: built.format,
            usage: built.usage,
            view_formats: built.view_formats,
        };
        let out = built.device.create_texture(&descriptor);
        Ok(out)
    }
    pub fn view(self) -> Result<TextureView> {
        let texture = self.make()?;
        let out = texture.create_view(&TextureViewDescriptor::default());
        Ok(out)
    }
}
