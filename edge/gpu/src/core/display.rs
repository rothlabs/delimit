use super::*;

const MIN_DIMENSION: u32 = 64;

#[derive(Debug)]
pub struct Display {
    inner: Grc<Surface<'static>>,
    device: Grc<Device>,
    pub adapter: Grc<Adapter>,
    format: TextureFormat,
    targets: Vec<Option<ColorTargetState>>,
    view_descriptor: TextureViewDescriptor<'static>,
    pub config: Leaf<SurfaceConfiguration>,
    // width: Leaf<u32>,
    // height: Leaf<u32>,
}

impl Display {
    pub fn new(inner: Grc<Surface<'static>>, adapter: Adapter, device: Grc<Device>) -> Self {
        // TODO: move this logic out to another function that winit uses
        let swapchain_capabilities = inner.get_capabilities(&adapter);
        let format = swapchain_capabilities.formats[0];
        let view_descriptor = TextureViewDescriptor::default();
        let width = 300;
        let height = 150;
        let config = inner.get_default_config(&adapter, width, height).unwrap();
        //inner.configure(&device, &config);
        Self {
            inner,
            device,
            adapter: adapter.into(),
            format,
            targets: vec![Some(format.into())],
            view_descriptor,
            config: config.into_leaf(),
            // width: width.into_leaf(),
            // height: height.into_leaf(),
        }
    }
    // pub fn resize(&self, width: u32, height: u32) -> graph::Result<()> {
    //     let config = self
    //         .inner
    //         .get_default_config(&self.adapter, width, height)
    //         .unwrap();
    //     self.inner.configure(&self.device, &config);
    //     Ok(())
    // }
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
    pub fn frame(&self) -> Result<SurfaceTexture> {
        Ok(self.inner.get_current_texture()?)
        // .expect("Failed to acquire next swap chain texture")
        // (frame, frame.texture.create_view(&self.view_descriptor))
    }
    // pub fn texture(&self) -> graph::Result<TextureBuilder> {
    //     let (width, height) = self.config.read(|config| (config.width, config.height))?;
    //     let size = Extent3d {
    //         width,
    //         height,
    //         depth_or_array_layers: 1,
    //     };
    //     Ok(TextureBuilder::default()
    //         .device(&self.device)
    //         .size(size)
    //         .usage(TextureUsages::RENDER_ATTACHMENT)
    //         .mip_level_count(1)
    //         .format(self.format))
    // }
}

// pub async fn resize(&self, width: u32, height: u32) -> graph::Result<()> {
//     // self.width.write(|x| *x = width.max(MIN_DIMENSION)).await?;
//     // self.height.write(|y| *y = height.max(MIN_DIMENSION)).await?;
//     self.config
//         .write(|config| {
//             config.width = width.max(MIN_DIMENSION);
//             config.height = height.max(MIN_DIMENSION);
//             self.inner.configure(&self.device, config);
//         })
//         .await
// }
