use super::*;

mod action;

pub trait ToViewport {
    fn viewport(self, core: Core, width: u32, height: u32) -> Result<Viewport>;
}

impl ToViewport for Surface<'static> {
    fn viewport(self, gpu: Core, width: u32, height: u32) -> Result<Viewport> {
        let swapchain_capabilities = self.get_capabilities(&gpu.adapter);
        let format = swapchain_capabilities.formats[0];
        let configuration = self
            .get_default_config(&gpu.adapter, width, height)
            .ok_or(anyhow!("no surface config"))?;
        self.configure(&gpu.device, &configuration);
        Ok(Viewport {
            gpu,
            surface: self.into(),
            configuration,
            targets: vec![Some(format.into())],
            format,
            chain: Leaf::default(),
            size: Leaf::new((width, height)),
        })
    }
}

#[derive(Clone, Debug)]
pub struct Viewport {
    pub gpu: Core,
    pub size: Leaf<(u32, u32)>,
    surface: Grc<Surface<'static>>,
    configuration: SurfaceConfiguration,
    targets: Vec<Option<ColorTargetState>>,
    format: TextureFormat,
    chain: Leaf<Vec<Command>>,
}

impl Viewport {
    pub fn shader(&self, source: ShaderModuleDescriptor) -> Shader {
        Shader {
            device: &self.gpu.device,
            module: self.gpu.device.create_shader_module(source),
            targets: &self.targets,
        }
    }
    pub fn pipe<'a>(&'a self, vertex: VertexState<'a>) -> pipe::RenderBuilder {
        pipe::RenderBuilder::default()
            .device(&self.gpu.device)
            .vertex(vertex)
    }
    pub fn command(&self) -> encode::render::CommandBuilder {
        encode::render::CommandBuilder::default().chain(self.chain.clone())
    }
    pub fn render(&self) -> Result<action::Render> {
        let chain = self.chain.base()?;
        Ok(action::Render {
            display: self,
            chain,
        })
    }
    pub fn frame(&self) -> Result<SurfaceTexture> {
        Ok(self.surface.get_current_texture()?)
    }
    pub fn resize(&mut self, width: u32, height: u32) {
        self.configuration.width = width;
        self.configuration.height = height;
        self.surface
            .configure(&self.gpu.device, &self.configuration);
    }
    pub fn texture(&self) -> Result<TextureBuilder> {
        //let (width, height) = self.config.read(|config| (config.width, config.height))?;
        // let (width, height) = self.configuration.width, self
        let (width, height) = self.size.base()?;
        let size = Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };
        Ok(TextureBuilder::default()
            .device(&self.gpu.device)
            .size(size)
            .usage(TextureUsages::RENDER_ATTACHMENT)
            .mip_level_count(1)
            .format(self.format))
    }
}
