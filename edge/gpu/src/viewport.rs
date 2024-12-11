use super::*;
use tokio::spawn;

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
            stage: stage(&gpu.device, format, width, height)?,
            gpu,
            size: Leaf::new((width, height)),
            surface: self.into(),
            config: configuration,
            targets: vec![Some(format.into())],
            format,
            commands: Leaf::default(),
        })
    }
}

// think about splitting this up into a viewport for things that care about size: Leaf<(u32, u32)> (main app)
// and things that care about surface, format, and render stage, number
// (some of these could be moved to gui::Display?)
#[derive(Clone, Debug)]
pub struct Viewport {
    // TODO: remove gpu: Core
    // Core can be replaced with Grc<Device>
    pub gpu: Core,
    pub size: Leaf<(u32, u32)>,
    pub commands: Leaf<Grc<Vec<flat::Command>>>,
    pub stage: Grc<TextureView>,
    surface: Grc<Surface<'static>>,
    config: SurfaceConfiguration,
    pub targets: Vec<Option<ColorTargetState>>,
    format: TextureFormat,
}

impl Viewport {
    pub fn shader(&self, source: ShaderModuleDescriptor) -> Shader {
        Shader {
            device: &self.gpu.device,
            module: self.gpu.device.create_shader_module(source),
            targets: &self.targets,
        }
    }
    pub fn pipe<'a>(&'a self, vertex: VertexState<'a>) -> pipe::render::RenderBuilder {
        pipe::render::RenderBuilder::default()
            .device(&self.gpu.device)
            .vertex(vertex)
    }
    // pub fn codec(&self) -> flat::render::CodecBuilder {
    //     flat::render::CodecBuilder::default()
    // }
    // pub fn pass(&self, steps: Hub<Vec<flat::render::Step>>) -> flat::render::pass::NodeBuilder {
    //     flat::render::pass::NodeBuilder::default().codec(steps)
    // }
    pub fn commands(&self, commands: Grc<Vec<flat::Command>>) -> Result<()> {
        self.commands.write_passive(|x| *x = commands)?;
        Ok(())
    }
    pub fn render(&self) -> Result<()> {
        action::Render {
            viewport: self,
            commands: self.commands.base()?,
        }
        .surface()
    }
    pub fn frame(&self) -> Result<SurfaceTexture> {
        Ok(self.surface.get_current_texture()?)
    }
    pub fn resize(&mut self, width: u32, height: u32) -> Result<()> {
        spawn(write_size(self.size.clone(), width, height));
        self.stage = stage(&self.gpu.device, self.format, width, height)?;
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.gpu.device, &self.config);
        Ok(())
    }
}

async fn write_size(size: Leaf<(u32, u32)>, width: u32, height: u32) -> Result<()> {
    size.write(|x| *x = (width, height)).await?;
    Ok(())
}

fn stage(
    device: &Device,
    format: TextureFormat,
    width: u32,
    height: u32,
) -> Result<Grc<TextureView>> {
    let size = Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };
    Ok(TextureBuilder::default()
        .device(device)
        .size(size)
        .usage(TextureUsages::RENDER_ATTACHMENT)
        .mip_level_count(1)
        .format(format)
        .sample_count(4)
        .view()?
        .into())
}

// pub fn command(&self) -> encode::render::CommandBuilder {
//     encode::render::CommandBuilder::default()//.chain(self.chain.clone())
// }

// async fn consume_chain(chain: Leaf<Vec<Command>>, number: u64) -> Result<()> {
//     chain.write(|chain| {
//         let index = chain.partition_point(|x| x.number == number);
//         // chain.spl
//         println!("chain and number: {} {}", chain.len(), number);
//         chain.drain(..index-1);
//         println!("after drain: {:?}", chain.len());
//         // *chain = chain[index..].to_vec();
//     }).await?;
//     Ok(())
// }

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
