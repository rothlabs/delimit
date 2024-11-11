use super::*;

pub struct Display {
    pub surface: Grc<Surface<'static>>,
    pub window: Grc<Window>,
}

impl Display {
    pub fn new(window: Window, gpu: Leaf<Option<Gpu>>) -> Self {
        let window = Grc::new(window);
        let instance = wgpu::Instance::default();
        let surface = Grc::new(instance.create_surface(window.clone()).unwrap());
        // let config = inner.get_default_config(adapter, width, height).unwrap();
        // inner.configure(&device, &config);

        // let gpu = self.gpu.clone();
        let display = Self {
            surface: surface.clone(),
            window,
        };
        tokio::task::spawn(async move {
            let core = Gpu::from_surface(instance, surface).await.unwrap();
            draw_triangle(&core).await.unwrap();
            gpu.write(|gpu| *gpu = Some(core)).await.unwrap();
        });
        display
    }
}
