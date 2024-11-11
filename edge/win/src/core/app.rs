use super::*;
use winit::dpi::PhysicalSize;

pub struct Display {
    pub configuration: Option<SurfaceConfiguration>,
    pub surface: Grc<Surface<'static>>,
    pub window: Grc<Window>,
}

impl Display {
    pub fn new(window: Window, gpu: Leaf<Option<Gpu>>) -> Result<Self> {
        let window = Grc::new(window);
        let instance = wgpu::Instance::default();
        let surface = Grc::new(instance.create_surface(window.clone())?);
        let display = Self {
            configuration: None,
            surface: surface.clone(),
            window,
        };
        tokio::task::spawn(make_gpu(gpu, instance, surface));
        Ok(display)
    }
    pub fn ensure(&mut self, gpu: &Gpu) -> Result<()> {
        if self.configuration.is_none() {
            let size = self.window.inner_size();
            let config = self
                .surface
                .get_default_config(&gpu.display.adapter, size.width, size.height)
                .ok_or(anyhow!("no surface config"))?;
            self.surface.configure(&gpu.device, &config);
            self.configuration = Some(config);
            self.window.set_visible(true);
        }
        Ok(())
    }
    pub fn resize(&mut self, gpu: &Gpu, size: PhysicalSize<u32>) {
        if let Some(config) = &mut self.configuration {
            config.width = size.width;
            config.height = size.height;
            self.surface.configure(&gpu.device, &config);
        }
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