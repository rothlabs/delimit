use winit::dpi::PhysicalSize;
use super::*;

pub struct Display {
    // gpu: Leaf<Option<Gpu>>,
    pub configured: bool,
    // pub size: PhysicalSize<u32>,
    pub surface: Grc<Surface<'static>>,
    pub window: Grc<Window>,
}

impl Display {
    pub fn new(window: Window, gpu: Leaf<Option<Gpu>>) -> Result<Self> {
        // let size = window.inner_size();
        let window = Grc::new(window);
        let instance = wgpu::Instance::default();
        let surface = Grc::new(instance.create_surface(window.clone())?);
        let display = Self {
            configured: false,
            // size,
            surface: surface.clone(),
            window,
        };
        tokio::task::spawn(make_gpu(gpu, instance, surface));
        Ok(display)
    }
    pub fn ensure(&mut self, gpu: &Gpu) -> Result<()> {
        if !self.configured {
            let size = self.window.inner_size();
            let config = self.surface.get_default_config(&gpu.display.adapter, size.width, size.height).ok_or(anyhow!("no surface config"))?;
            self.surface.configure(&gpu.device, &config);
            self.configured = true;
        }
        Ok(())
    }
    pub fn resize(&mut self, size: PhysicalSize) {
        // if let Some(gpu) = self.gpu.base().unwrap() {
    }
}

async fn make_gpu(
    gpu: Leaf<Option<Gpu>>,
    instance: Instance,
    surface: Grc<Surface<'static>>,
) -> Result<()> {
    let core = Gpu::from_surface(instance, surface).await?;
    draw_triangle(&core).await?;
    gpu.write(|gpu| *gpu = Some(core)).await?;
    Ok(())
}


// pub async fn initialize(&self,
//     // gpu: Leaf<Option<Gpu>>,
//     // instance: Instance,
//     // surface: Grc<Surface<'static>>,
// ) -> Result<()> {
//     let core = Gpu::from_surface(&self.instance, self.surface.clone()).await?;
//     draw_triangle(&core).await?;
//     self.gpu.write(|gpu| *gpu = Some(core)).await?;
//     Ok(())
// }