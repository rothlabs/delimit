use super::*;
use tokio::task::spawn;

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
        let stage = gpu.render_stage(format, width, height)?;
        Ok(Viewport {
            gpu,
            size: Leaf::new((width, height)),
            surface: self.into(),
            configuration,
            targets: vec![Some(format.into())],
            format,
            stage: stage.into(),
            chain: Leaf::new(vec![Command::default()]),
            number: 0,
        })
    }
}

// think about splitting this up into a viewport for things that care about size: Leaf<(u32, u32)> (main app)
// and things that care about surface, format, and render stage, number
// (some of these could be moved to gui::Display)
#[derive(Clone, Debug)]
pub struct Viewport {
    pub gpu: Core,
    pub size: Leaf<(u32, u32)>,
    surface: Grc<Surface<'static>>,
    configuration: SurfaceConfiguration,
    targets: Vec<Option<ColorTargetState>>,
    format: TextureFormat,
    stage: Grc<TextureView>,
    chain: Leaf<Vec<Command>>,
    number: u64,
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
    pub fn render(&mut self) -> Result<()> {
        let chain = self.chain.base()?;
        let number = chain.last().ok_or(anyhow!("no commands"))?.number;
        spawn(consume_chain(self.chain.clone(), number));
        // TODO: change action::Render to be a function
        let out = action::Render {
            display: self,
            chain,
        }.surface();
        self.number = number;
        out
    }
    pub fn frame(&self) -> Result<SurfaceTexture> {
        Ok(self.surface.get_current_texture()?)
    }
    pub fn resize(&mut self, width: u32, height: u32) -> Result<()> {
        println!("resize {} {}", width, height);
        self.stage = self.gpu.render_stage(self.format, width, height)?.into();
        self.configuration.width = width;
        self.configuration.height = height;
        self.surface
            .configure(&self.gpu.device, &self.configuration);
        Ok(())
    }
}

async fn consume_chain(chain: Leaf<Vec<Command>>, number: u64) -> Result<()> {
    chain.write(|chain| {
        let index = chain.partition_point(|x| x.number == number);
        // chain.spl
        println!("chain and number: {} {}", chain.len(), number);
        chain.drain(..index-1);
        println!("after drain: {:?}", chain.len());
        // *chain = chain[index..].to_vec();
    }).await?;
    Ok(())
}


// pub fn texture(&self) -> Result<TextureBuilder> {
//     println!("texture {} {}", self.configuration.width, self.configuration.height);
//     //let (width, height) = self.config.read(|config| (config.width, config.height))?;
//     // let (width, height) = self.configuration.width, self
//     // let (width, height) = self.size.base()?;
//     let size = Extent3d {
//         width: self.configuration.width,
//         height: self.configuration.height,
//         depth_or_array_layers: 1,
//     };
//     Ok(TextureBuilder::default()
//         .device(&self.gpu.device)
//         .size(size)
//         .usage(TextureUsages::RENDER_ATTACHMENT)
//         .mip_level_count(1)
//         .format(self.format))
// }