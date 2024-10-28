use super::*;

#[derive(Debug)]
pub struct Display {
    inner: Surface<'static>,
    device: Grc<Device>,
    // adapter: Grc<Adapter>,
    format: TextureFormat,
    targets: Vec<Option<ColorTargetState>>,
    view_descriptor: TextureViewDescriptor<'static>,
    // config: SurfaceConfiguration,
    // width: u32,
    // height: u32
    config: Leaf<SurfaceConfiguration>
}

impl Display {
    pub fn new(inner: Surface<'static>, adapter: &Adapter, device: Grc<Device>) -> Self {
        let swapchain_capabilities = inner.get_capabilities(adapter);
        let format = swapchain_capabilities.formats[0];
        let view_descriptor = TextureViewDescriptor::default();
        // let width = 300;
        // let height = 150;
        let config = inner.get_default_config(&adapter, 300, 150).unwrap();
        inner.configure(&device, &config);
        Self {
            inner,
            device,
            // adapter,
            format,
            targets: vec![Some(format.into())],
            view_descriptor,
            config: Leaf::new(config),
        }
    }
    pub async fn resize(&self, width: u32, height: u32) {
        // self.config.width = width.max(1);
        // self.config.height = height.max(1);
        // let config = self.inner.get_default_config(&self.adapter, width, height).unwrap();
        // TODO: writing to a leaf should not be a future!
        self.config.write(|config| {
            config.width = width.max(1);
            config.height = height.max(1);
            self.inner.configure(&self.device, config);
        }).await.ok();
    }
    pub fn targets(&self) -> &[Option<ColorTargetState>] {
        &self.targets
    }
    pub fn view(&self) -> TextureView {
        let frame = self
            .inner
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        frame.texture.create_view(&self.view_descriptor)
    }
    pub fn texture(&self) -> graph::Result<TextureBuilder> {
        let (width, height) = self.config.read(|config| (config.width, config.height))?;
        let size = Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };
        Ok(TextureBuilder::default()
            .device(&self.device)
            .size(size)
            .usage(TextureUsages::RENDER_ATTACHMENT)
            .mip_level_count(1)
            .format(self.format))
    }
}

// #[derive(Debug)]
// pub struct DisplayConfig {
//     width: u32,
//     height: u32,
// }

// impl Act for Display {
//     async fn act(&self) -> graph::Result<()> {
//         Ok(())
//     }
// }

// pub fn fragment(&'a self, shader: &'a ShaderModule) -> FragmentBuilder<'a> {
//     FragmentBuilder::default()
//         .module(shader)
//         .targets(&self.targets)
// }
