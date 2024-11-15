use super::*;
use winit::dpi::PhysicalSize;

#[derive(Default, Clone, Debug)]
pub struct Agent {
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
        println!("wrote displays");
        window.set_visible(true);
        Ok(())
    }
    pub fn render(&self, id: WindowId) -> Result<()> {
        self.displays.read(|displays| {
            for display in displays {
                if display.window.id() == id {
                    return display.render();
                }
            }
            Err(anyhow!("no display of window id for rendering"))?
        })?
    }
    pub fn resize(&mut self, id: WindowId, size: PhysicalSize<u32>) -> Result<()> {
        self.displays.write_passive(|displays| {
            for display in displays {
                if display.window.id() == id {
                    return display.resize(size);
                }
            }
            Err(anyhow!("no display of given window id for resize"))?
        })?
    }
    pub fn is_empty(&self) -> Result<bool> {
        Ok(self.displays.base()?.is_empty())
    }
}

// trait GetDisplay {
//     fn get(&mut self, id: WindowId) -> Result<&mut Display>;
// }

// impl GetDisplay for Vec<Display> {
//     fn get(&mut self, id: WindowId) -> Result<&mut Display> {
//         for display in self {
//             if display.window.id() == id {
//                 return Ok(display);
//             }
//         }
//         Err(anyhow!("no display of given window id"))?
//     }
// }

// pub fn resize(&mut self, id: WindowId, size: PhysicalSize<u32>) -> Result<()> {
//     // self.displays_base = self.displays.base()?;
//     // self.get_display(id)?.resize(size)?;
//     self.displays.write_passive(|x| x.get(id)?.resize(size))??;
//     Ok(())
// }

// fn get_display(&self, id: WindowId) -> Result<Display> {
//     for display in &self.displays.base()? {
//         if display.window.id() == id {
//             return Ok(display.clone());
//         }
//     }
//     Err(anyhow!("no display of given window id"))?
// }

// fn get_display(&self, id: WindowId) -> Result<Display> {
//     for display in self.displays.base()? {
//         if display.window.id() == id {
//             return Ok(display);
//         }
//     }
//     Err(anyhow!("no display of given window id"))?
// }

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
