use super::*;
use winit::dpi::PhysicalSize;

pub struct Display {
    pub configuration: Option<SurfaceConfiguration>,
    pub surface: Grc<Surface<'static>>,
    pub window: Grc<Window>,
}

impl Display {
    pub fn new(gpu: Leaf<Option<Gpu>>, window: Window) -> Result<Self> {
        let window = Grc::new(window);
        let instance = wgpu::Instance::default();
        let surface = Grc::new(instance.create_surface(window.clone())?);
        let display = Self {
            configuration: None,
            surface: surface.clone(),
            window: window.clone(),
        };
        tokio::task::spawn(make_gpu(gpu, instance, surface, window));
        Ok(display)
    }
    pub fn ensure_configuration(&mut self, gpu: &Gpu) -> Result<()> {
        if self.configuration.is_none() {
            let size = self.window.inner_size();
            let config = self
                .surface
                .get_default_config(&gpu.adapter, size.width, size.height)
                .ok_or(anyhow!("no surface config"))?;
            self.surface.configure(&gpu.device, &config);
            self.configuration = Some(config);
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
    window: Grc<Window>,
) -> Result<()> {
    let core = Gpu::from_surface(instance, surface).await?;
    draw_triangle(&core).await?;
    gpu.write(|gpu| *gpu = Some(core)).await?;
    window.set_visible(true);
    Ok(())
}

async fn draw_triangle(gpu: &Gpu) -> gpu::Result<()> {
    let targets = gpu.display.targets();
    let shader = gpu.shader(include_wgsl!("triangle.wgsl"));
    let vertex = shader.vertex("vs_main").make()?;
    let fragment = shader.fragment("fs_main").targets(targets).make()?;
    let pipe = gpu.render_pipe(vertex).fragment(fragment).make()?;
    gpu.command()
        .render(pipe)
        .draw(0..3, 0..1)
        .hub()?
        .base()
        .await?;
    Ok(())
}
