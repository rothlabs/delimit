use super::*;
use winit::dpi::PhysicalSize;

// pub type Displays = Leaf<Vec<Display>>;

#[derive(Default, Clone, Debug)]
pub struct Displays(Leaf<Vec<Display>>);

impl Displays {
    pub async fn main(self, window: Grc<Window>) -> Result<()> {
        let size = window.inner_size();
        // TODO: make instance an argument
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window.clone())?;
        let gpu = instance.surface_adapter(&surface).await?.gpu().await?;
        let viewport = surface.viewport(gpu, size.width, size.height)?;
        draw_triangle(&viewport).await?;
        let display = Display {
            viewport,
            window: window.clone(),
        };
        self.0.write(|x| x.push(display)).await?;
        window.set_visible(true);
        Ok(())
    }
    pub fn render(&self, id: WindowId) -> Result<()> {
        self.display(id)?.render()
    }
    pub fn resize(&self, id: WindowId, size: PhysicalSize<u32>) -> Result<()> {
        self.display(id)?.resize(size);
        Ok(())
    }
    pub fn is_empty(&self) -> Result<bool> {
        Ok(self.0.base()?.is_empty())
    }
    fn display(&self, id: WindowId) -> Result<Display> {
        for display in self.0.base()? {
            if display.window.id() == id {
                return Ok(display);
            } 
        }
        Err(anyhow!("no display of given window id"))?
    }
}

#[derive(Clone, Debug)]
pub struct Display {
    viewport: Viewport,
    window: Grc<Window>,
}

impl Display {
    fn render(&self) -> Result<()> {
        Ok(self.viewport.render()?.surface()?)
    }
    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.viewport.resize(size.width, size.height);
    }
}

async fn draw_triangle(viewport: &Viewport) -> gpu::Result<()> {
    // let targets = viewport.targets;
    let shader = viewport.shader(include_wgsl!("triangle.wgsl"));
    let vertex = shader.vertex("vs_main").make()?;
    let fragment = shader.fragment("fs_main");
    // TODO: move render pipe to viewport and make it take fragment builder so it can add targets
    let pipe = viewport.pipe(vertex).fragment(fragment).make()?;
    viewport.command()
        .render(pipe)
        .draw(0..3, 0..1)
        .hub()?
        .base()
        .await?;
    Ok(())
}

// pub fn new(gpu: Leaf<Option<Gpu>>, window: Window) -> Result<Self> {
    //     let window = Grc::new(window);
    //     let instance = wgpu::Instance::default();
    //     let surface = Grc::new(instance.create_surface(window.clone())?);
    //     let display = Self {
    //         // configuration: None,
    //         // surface: surface.clone(),
    //         window: window.clone(),
    //     };
    //     tokio::task::spawn(make_gpu(gpu, instance, surface, window));
    //     Ok(display)
    // }
    // pub fn ensure_configuration(&mut self) -> Result<()> {
    //     Ok(())
    //     // let size = self.window.inner_size();
    //     // Ok(self.viewport.ensure_configuration(size.width, size.height)?)
    //     // //if let Some(viewport) = &self.viewport {
    //     //     // viewport.s
    //     // //}
    //     // Ok(())
    // }

// async fn make_gpu(
//     gpu: Leaf<Option<Gpu>>,
//     instance: Instance,
//     surface: Grc<Surface<'static>>,
//     window: Grc<Window>,
// ) -> Result<()> {
//     let core = instance.surface_adapter(surface).await?.gpu().await?;
//     // draw_triangle(&core).await?;
//     gpu.write(|gpu| *gpu = Some(core)).await?;
//     window.set_visible(true);
//     Ok(())
// }

// async fn draw_triangle(gpu: &Gpu) -> gpu::Result<()> {
//     let targets = gpu.display.targets();
//     let shader = gpu.shader(include_wgsl!("triangle.wgsl"));
//     let vertex = shader.vertex("vs_main").make()?;
//     let fragment = shader.fragment("fs_main").targets(targets).make()?;
//     let pipe = gpu.render_pipe(vertex).fragment(fragment).make()?;
//     gpu.command()
//         .render(pipe)
//         .draw(0..3, 0..1)
//         .hub()?
//         .base()
//         .await?;
//     Ok(())
// }




// async fn make_gpu(
//     gpu: Leaf<Option<Gpu>>,
//     instance: Instance,
//     surface: Grc<Surface<'static>>,
//     window: Grc<Window>,
// ) -> Result<()> {
//     let core = Gpu::from_surface(instance, surface).await?;
//     draw_triangle(&core).await?;
//     gpu.write(|gpu| *gpu = Some(core)).await?;
//     window.set_visible(true);
//     Ok(())
// }


// pub fn new(gpu: Leaf<Option<Gpu>>, window: Window) -> Result<Self> {
//     let window = Grc::new(window);
//     let instance = wgpu::Instance::default();
//     let surface = Grc::new(instance.create_surface(window.clone())?);
//     let display = Self {
//         configuration: None,
//         surface: surface.clone(),
//         window: window.clone(),
//     };
//     tokio::task::spawn(make_gpu(gpu, instance, surface, window));
//     Ok(display)
// }


// pub fn ensure_configuration(&mut self, gpu: &Gpu) -> Result<()> {
//     if self.configuration.is_none() {
//         let size = self.window.inner_size();
//         let config = self
//             .surface
//             .get_default_config(&gpu.adapter, size.width, size.height)
//             .ok_or(anyhow!("no surface config"))?;
//         self.surface.configure(&gpu.device, &config);
//         self.configuration = Some(config);
//     }
//     Ok(())
// }