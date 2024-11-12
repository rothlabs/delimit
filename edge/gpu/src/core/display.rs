use super::*;

// const MIN_DIMENSION: u32 = 64;

#[derive(Debug)]
pub struct Display {
    surface: Grc<Surface<'static>>,
    // pub adapter: Grc<Adapter>,
    pub targets: Vec<Option<ColorTargetState>>,
    // view_descriptor: TextureViewDescriptor<'static>,
    // pub config: Leaf<SurfaceConfiguration>,
}

impl Display {
    pub fn new(surface: Grc<Surface<'static>>, adapter: &Adapter) -> Self {
        // TODO: move this logic out to another function that winit uses
        let swapchain_capabilities = surface.get_capabilities(adapter);
        let format = swapchain_capabilities.formats[0];
        // let view_descriptor = TextureViewDescriptor::default();
        Self {
            surface,
            // adapter: adapter.into(),
            targets: vec![Some(format.into())],
            // view_descriptor,
        }
    }
    pub fn targets(&self) -> &[Option<ColorTargetState>] {
        &self.targets
    }
    pub fn view(&self) -> TextureView {
        let frame = self
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        frame.texture.create_view(&TextureViewDescriptor::default())
    }
    pub fn frame(&self) -> Result<SurfaceTexture> {
        Ok(self.surface.get_current_texture()?)
    }
}

// #[derive(Debug)]
// pub struct Display {
//     inner: Grc<Surface<'static>>,
//     device: Grc<Device>,
//     pub adapter: Grc<Adapter>,
//     format: TextureFormat,
//     targets: Vec<Option<ColorTargetState>>,
//     view_descriptor: TextureViewDescriptor<'static>,
//     pub config: Leaf<SurfaceConfiguration>,
// }

// impl Display {
//     pub fn new(inner: Grc<Surface<'static>>, adapter: Adapter, device: Grc<Device>) -> Self {
//         // TODO: move this logic out to another function that winit uses
//         let swapchain_capabilities = inner.get_capabilities(&adapter);
//         let format = swapchain_capabilities.formats[0];
//         let view_descriptor = TextureViewDescriptor::default();
//         let width = 300;
//         let height = 150;
//         let config = inner.get_default_config(&adapter, width, height).unwrap();
//         //inner.configure(&device, &config);
//         Self {
//             inner,
//             device,
//             adapter: adapter.into(),
//             format,
//             targets: vec![Some(format.into())],
//             view_descriptor,
//             config: config.into_leaf(),
//         }
//     }
//     pub fn targets(&self) -> &[Option<ColorTargetState>] {
//         &self.targets
//     }
//     pub fn view(&self) -> TextureView {
//         let frame = self
//             .inner
//             .get_current_texture()
//             .expect("Failed to acquire next swap chain texture");
//         frame.texture.create_view(&self.view_descriptor)
//     }
//     pub fn frame(&self) -> Result<SurfaceTexture> {
//         Ok(self.inner.get_current_texture()?)
//         // .expect("Failed to acquire next swap chain texture")
//         // (frame, frame.texture.create_view(&self.view_descriptor))
//     }
//     // pub fn texture(&self) -> graph::Result<TextureBuilder> {
//     //     let (width, height) = self.config.read(|config| (config.width, config.height))?;
//     //     let size = Extent3d {
//     //         width,
//     //         height,
//     //         depth_or_array_layers: 1,
//     //     };
//     //     Ok(TextureBuilder::default()
//     //         .device(&self.device)
//     //         .size(size)
//     //         .usage(TextureUsages::RENDER_ATTACHMENT)
//     //         .mip_level_count(1)
//     //         .format(self.format))
//     // }
// }
