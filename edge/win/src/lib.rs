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
            let surface = instance.create_surface(window).unwrap();
            let gpu = self.gpu.clone();
            let chain = self.chain.clone();
            tokio::task::spawn(async move {
                let gpu_core = Gpu::from_surface(instance, surface, chain).await.unwrap();
                gpu.write(|gpu| *gpu = Some(gpu_core)).await.unwrap();

            });
        }
    }
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                println!("request redraw");
                if let Some(gpu) = self.gpu.read(|gpu| gpu.clone()).unwrap() {
                    // let chain = self.chain.read(|x| x.clone()).unwrap();
                    gpu.render();
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
                    // println!("tokio mouse delta: {:?}", delta);
                    let _ = events.x.write(|x| *x = delta.0).await;
                });
            }
            _ => (),
        }
    }
}

async fn draw_triangle(gpu: Gpu) -> gpu::Result<()> {
    let targets = gpu.display.targets();
    let shader = gpu.shader(include_wgsl!("triangle.wgsl"));
    let vertex = shader.vertex("vs_main").make()?;
    let fragment = shader.fragment("fs_main").targets(targets).make()?;
    let pipe = gpu.render_pipe(vertex).fragment(fragment).make()?;
    // let view = gpu.display.view();
    // gpu.command()
    //     .display(gpu.display.clone())
    //     // .texture_view(view)
    //     .render(pipe)
    //     .draw(0..3, 0..1)
    //     .hub()?
    //     .base()
    //     .await?;
    println!("draw triangle complete");
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
