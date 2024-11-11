use graph::*;
use gpu::*;
use wgpu::*;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

pub type Win = Core;

// #[derive(Default)]
pub struct Core {
    // pub window: Option<Grc<Box<Window>>>,
    // pub window: Leaf<Option<Box<Window>>>,
    events: EventsLeaf,
    start: Box<dyn Fn(Instance, Surface<'static>)>,
    gpu: Leaf<Option<Gpu>>,
    window: Option<Grc<Window>>,
}

impl Core {
    pub fn new(start: Box<dyn Fn(Instance, Surface<'static>)>) -> Self { // gpu: Leaf<Option<Gpu>>, 
        Self {
            gpu: Leaf::new(None),
            window: None,
            events: EventsLeaf::default(),
            start,
        }
    }
}

// impl Default for Core {
//     fn default() -> Self {
//         Self { window: Leaf::new(None), events: EventsLeaf::default() }
//     }
// }
async fn draw_triangle(gpu: Leaf<Option<Gpu>>) -> gpu::Result<()> {
    if let Some(gpu) = gpu.read(|gpu| gpu.clone())? {
        let targets = gpu.display.targets();
        let shader = gpu.shader(include_wgsl!("triangle.wgsl"));
        let vertex = shader.vertex("vs_main").make()?;
        let fragment = shader.fragment("fs_main").targets(targets).make()?;
        let pipe = gpu.render_pipe(vertex).fragment(fragment).make()?;
        // let view = gpu.display.view();
        gpu.command()
            .display(gpu.display.clone())
            // .texture_view(view)
            .render(pipe)
            .draw(0..3, 0..1)
            .hub()?
            .base()
            .await?;
        println!("draw triangle complete");
    }
    Ok(())
}


impl ApplicationHandler for Core {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window = Grc::new(event_loop
                .create_window(Window::default_attributes())
                .unwrap());
            self.window = Some(window.clone());
            let instance = Instance::default();
            let surface = instance.create_surface(window).unwrap();
            let gpu = self.gpu.clone();
            tokio::task::spawn(async move {
                let gpu_core = Gpu::from_surface(instance, surface).await.unwrap();
                gpu.write(|gpu| *gpu = Some(gpu_core)).await.unwrap();
                //draw_triangle(gpu).await.unwrap();
            });
            // (self.start)(instance, surface);
        }
    }
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                println!("request redraw");
                let gpu = self.gpu.clone();
                tokio::task::spawn(async move {
                    
                    draw_triangle(gpu).await.unwrap();
                    println!("done drawinng");
                });
                println!("spawned draw task");
                // self.window.as_ref().unwrap().request_redraw();
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

#[derive(Clone)]
pub struct EventsLeaf {
    x: Leaf<f64>,
}

impl Default for EventsLeaf {
    fn default() -> Self {
        Self { x: Leaf::new(0.) }
    }
}
