use super::*;

mod action;

pub trait ToViewport {
    fn display(&self, core: Core) -> Viewport;
}

impl ToViewport for Grc<Surface<'static>> {
    fn display(&self, core: Core) -> Viewport {
        let swapchain_capabilities = self.get_capabilities(&core.adapter);
        let format = swapchain_capabilities.formats[0];
        Viewport {
            core,
            surface: self.clone(),
            configuration: None,
            targets: vec![Some(format.into())],
            chain: Leaf::default(),
        }
    }
}

pub struct Viewport {
    core: Core,
    surface: Grc<Surface<'static>>,
    configuration: Option<SurfaceConfiguration>,
    targets: Vec<Option<ColorTargetState>>,
    chain: Leaf<Vec<Command>>,
}

impl Viewport {
    pub fn ensure_configuration(&mut self, width: u32, height: u32) -> Result<()> {
        if self.configuration.is_none() {
            // let size = self.window.inner_size();
            let config = self
                .surface
                .get_default_config(&self.core.adapter, width, height)
                .ok_or(anyhow!("no surface config"))?;
            self.surface.configure(&self.core.device, &config);
            self.configuration = Some(config);
        }
        Ok(())
    }
    pub fn command(&self) -> encode::render::CommandBuilder {
        encode::render::CommandBuilder::default().chain(self.chain.clone())
    }
    pub fn render(&self) -> action::Render {
        let chain = self.chain.read(|x| x.clone()).unwrap();
        action::Render { display: self, chain }
    }
    pub fn frame(&self) -> Result<SurfaceTexture> {
        Ok(self.surface.get_current_texture()?)
    }
}
