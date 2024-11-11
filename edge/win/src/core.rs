use super::*;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

mod app;

#[derive(Default)]
pub struct App {
    events: EventsLeaf,
    gpu: Leaf<Option<Gpu>>,
    display: Option<app::Display>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.display.is_none() {
            let fields = Window::default_attributes();
            let window = event_loop.create_window(fields).unwrap();
            self.display = Some(app::Display::new(window, self.gpu.clone()));
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
                    // let adapter = &gpu.display.adapter;
                    // let config = self.surface.get_default_config(&self.adapter, width, height).unwrap();
                    // self.inner.configure(&self.device, &config);
                    gpu.display.resize(new_size.width, new_size.height).unwrap();
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(gpu) = self.gpu.read(|gpu| gpu.clone()).unwrap() {
                    gpu.render().surface().unwrap();
                }
            }
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
