use super::*;
use winit::dpi::PhysicalSize;

#[derive(Default, Clone, Debug)]
pub struct Agent {
    // pub gpu: Gpu,
    pub instance: Grc<Instance>,
    pub displays: Leaf<Vec<Display>>,
}

impl Agent {
    pub async fn display(self, window: Grc<Window>) -> Result<()> {
        let surface = self.instance.create_surface(window.clone())?;
        let displays = self.displays.base()?;
        if let Some(main) = displays.first() {
            let gpu = main.viewport.gpu.clone();
            self.make_display(window, surface, gpu).await
        } else {
            self.make_primary(window, surface).await
        }
    }
    async fn make_primary(self, window: Grc<Window>, surface: Surface<'static>) -> Result<()> {
        let gpu = self.instance.surface_adapter(&surface).await?.gpu().await?;
        self.make_display(window, surface, gpu).await
    }
    async fn make_display(
        &self,
        window: Grc<Window>,
        surface: Surface<'static>,
        gpu: Gpu,
    ) -> Result<()> {
        let size = window.inner_size();
        let viewport = surface.viewport(gpu, size.width, size.height)?;
        // post_triangle(&viewport).await?;
        let display = Display::new(window.clone(), viewport);
        self.displays.write(|x| x.push(display)).await?;
        window.set_visible(true);
        Ok(())
    }
    pub fn render(&self, id: WindowId) -> Result<()> {
        self.get_display(id)?.render()
    }
    pub fn resize(&self, id: WindowId, size: PhysicalSize<u32>) -> Result<()> {
        self.get_display(id)?.resize(size)?;
        Ok(())
    }
    pub fn is_empty(&self) -> Result<bool> {
        Ok(self.displays.base()?.is_empty())
    }
    fn get_display(&self, id: WindowId) -> Result<Display> {
        for display in self.displays.base()? {
            if display.window.id() == id {
                return Ok(display);
            }
        }
        Err(anyhow!("no display of given window id"))?
    }
}

// async fn post_triangle(viewport: &Viewport) -> gpu::Result<()> {
//     let shader = viewport.shader(include_wgsl!("triangle.wgsl"));
//     let vertex = shader.vertex("vs_main").make()?;
//     let fragment = shader.fragment("fs_main").make()?;
//     let pipe = viewport.pipe(vertex).fragment(fragment).make()?;
//     viewport
//         .command()
//         .render(pipe)
//         .draw(0..3, 0..1)
//         .hub()?
//         .base()
//         .await?;
//     Ok(())
// }
