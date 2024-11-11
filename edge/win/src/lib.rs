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
