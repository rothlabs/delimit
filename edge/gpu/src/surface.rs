use super::*;

// #[derive(Debug)]
pub struct Surface<'a> {
    inner: wgpu::Surface<'a>,
    targets: Vec<Option<ColorTargetState>>,
    view_descriptor: TextureViewDescriptor<'a>,
    config: SurfaceConfiguration,
}

impl<'a> Surface<'a> {
    pub fn new(inner: wgpu::Surface<'a>, adapter: &Adapter, device: &Device) -> Self {
        let swapchain_capabilities = inner.get_capabilities(adapter);
        let format = swapchain_capabilities.formats[0];
        let view_descriptor = TextureViewDescriptor::default();
        let config = inner.get_default_config(adapter, 300, 150).unwrap();
        inner.configure(device, &config);
        Self {
            inner,
            targets: vec![Some(format.into())],
            view_descriptor,
            config,
        }
    }
    pub fn targets(&'a self) -> &'a [Option<ColorTargetState>] {
        &self.targets
    }
    // pub fn fragment(&'a self, shader: &'a ShaderModule) -> FragmentBuilder<'a> {
    //     FragmentBuilder::default()
    //         .module(shader)
    //         .targets(&self.targets)
    // }
    pub fn view(&self) -> TextureView {
        let frame = self
            .inner
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        frame.texture.create_view(&self.view_descriptor)
    }
    pub fn resize(&mut self, width: u32, height: u32) {
        self.config.width = width.max(1);
        self.config.height = height.max(1);
        // surface.configure(&device, &config);
    }
}
