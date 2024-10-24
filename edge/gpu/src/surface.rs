use super::*;

pub struct Surface<'a> {
    inner: wgpu::Surface<'a>,
    device: Grc<Device>,
    format: TextureFormat,
    targets: Vec<Option<ColorTargetState>>,
    view_descriptor: TextureViewDescriptor<'a>,
    config: SurfaceConfiguration,
}

impl<'a> Surface<'a> {
    pub fn new(inner: wgpu::Surface<'a>, adapter: &Adapter, device: Grc<Device>) -> Self {
        let swapchain_capabilities = inner.get_capabilities(adapter);
        let format = swapchain_capabilities.formats[0];
        let view_descriptor = TextureViewDescriptor::default();
        let config = inner.get_default_config(adapter, 300, 150).unwrap();
        inner.configure(&device, &config);
        Self {
            inner,
            device,
            format,
            targets: vec![Some(format.into())],
            view_descriptor,
            config,
        }
    }
    pub fn resize(&mut self, width: u32, height: u32) {
        self.config.width = width.max(1);
        self.config.height = height.max(1);
        // surface.configure(&device, &config);
    }
    pub fn targets(&'a self) -> &'a [Option<ColorTargetState>] {
        &self.targets
    }
    pub fn view(&self) -> TextureView {
        let frame = self
            .inner
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        frame.texture.create_view(&self.view_descriptor)
    }
    pub fn texture(&self) -> TextureBuilder {
        let size = Extent3d {
            width: self.config.width,
            height: self.config.height,
            depth_or_array_layers: 1,
        };
        TextureBuilder::default()
            .device(&self.device)
            .size(size)
            .usage(TextureUsages::RENDER_ATTACHMENT)
            .mip_level_count(1)
            .format(self.format)
    }
}

// pub fn fragment(&'a self, shader: &'a ShaderModule) -> FragmentBuilder<'a> {
//     FragmentBuilder::default()
//         .module(shader)
//         .targets(&self.targets)
// }
