use super::*;
use winit::dpi::PhysicalSize;

#[derive(Default, Clone, Debug)]
pub struct Core(Leaf<Vec<Display>>);

impl Core {
    pub async fn main(self, window: Grc<Window>) -> Result<()> {
        let size = window.inner_size();
        // TODO: make instance an argument
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window.clone())?;
        let gpu = instance.surface_adapter(&surface).await?.gpu().await?;
        let viewport = surface.viewport(gpu, size.width, size.height)?;
        post_triangle(&viewport).await?;
        let display = Display::new(&window, viewport);
        self.0.write(|x| x.push(display)).await?;
        window.set_visible(true);
        Ok(())
    }
    pub async fn display(self, window: Grc<Window>) -> Result<()> {
        let size = window.inner_size();
        let displays = self.0.base()?;
        let main = displays.first().ok_or(anyhow!("no main display"))?;
        let gpu = main.viewport.gpu.clone();
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window.clone())?;
        let viewport = surface.viewport(gpu, size.width, size.height)?;
        post_triangle(&viewport).await?;
        let display = Display::new(&window, viewport);
        self.0.write(|x| x.push(display)).await?;
        window.set_visible(true);
        Ok(())
    }
    pub fn render(&self, id: WindowId) -> Result<()> {
        self.get(id)?.render()
    }
    pub fn resize(&self, id: WindowId, size: PhysicalSize<u32>) -> Result<()> {
        self.get(id)?.resize(size);
        Ok(())
    }
    pub fn is_empty(&self) -> Result<bool> {
        Ok(self.0.base()?.is_empty())
    }
    fn get(&self, id: WindowId) -> Result<Display> {
        for display in self.0.base()? {
            if display.window.id() == id {
                return Ok(display);
            }
        }
        Err(anyhow!("no display of given window id"))?
    }
}

async fn post_triangle(viewport: &Viewport) -> gpu::Result<()> {
    let shader = viewport.shader(include_wgsl!("triangle.wgsl"));
    let vertex = shader.vertex("vs_main").make()?;
    let fragment = shader.fragment("fs_main").make()?;
    let pipe = viewport.pipe(vertex).fragment(fragment).make()?;
    viewport
        .command()
        .render(pipe)
        .draw(0..3, 0..1)
        .hub()?
        .base()
        .await?;
    Ok(())
}