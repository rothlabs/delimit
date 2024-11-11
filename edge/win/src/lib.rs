use graph::*;
use gpu::*;
use wgpu::*;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

mod render;

#[derive(Default)]
pub struct App {
    events: EventsLeaf,
    gpu: Leaf<Option<Gpu>>,
    chain: Leaf<Vec<Command>>,
    // rendered: bool,
    surface: Option<Grc<Surface<'static>>>,
    window: Option<Grc<Window>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window = Grc::new(event_loop
                .create_window(Window::default_attributes())
                .unwrap());
            self.window = Some(window.clone());
            let instance = Instance::default();
            let surface = Grc::new(instance.create_surface(window).unwrap());
            // let config = inner.get_default_config(adapter, width, height).unwrap();
            // inner.configure(&device, &config);
            self.surface = Some(surface.clone());
            let gpu = self.gpu.clone();
            let chain = self.chain.clone();
            tokio::task::spawn(async move {
                let core = Gpu::from_surface(instance, surface, chain).await.unwrap();
                draw_triangle(&core).await.unwrap();
                gpu.write(|gpu| *gpu = Some(core)).await.unwrap();
            });
        }
    }
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(new_size) => {
                // Reconfigure the surface with the new size
                if let Some(gpu) = self.gpu.read(|gpu| gpu.clone()).unwrap() {
                    gpu.display.resize2(new_size.width, new_size.height).unwrap();
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(gpu) = self.gpu.read(|gpu| gpu.clone()).unwrap() {
                    gpu.render().surface();
                }
                
            }
            // WindowEvent::CursorMoved { device_id, position } => {

            // },
            _ => (),
        }
    }
    fn device_event(
        &mut self,
        _: &ActiveEventLoop,
        _: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        match event {
            DeviceEvent::MouseMotion { delta } => {
                let events = self.events.clone();
                tokio::task::spawn(async move {
                    let _ = events.x.write(|x| *x = delta.0).await;
                });
            }
            _ => (),
        }
    }
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
    // println!("draw triangle complete");
    Ok(())
}



#[derive(Clone)]
pub struct EventsLeaf {
    x: Leaf<f64>,
}

impl Default for EventsLeaf {
    fn default() -> Self {
        Self { x: Leaf::new(0.) }
    }
}
