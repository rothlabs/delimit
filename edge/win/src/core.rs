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
            let fields = Window::default_attributes().with_visible(false);
            let window = event_loop.create_window(fields).unwrap();
            let display = app::Display::new(self.gpu.clone(), window).unwrap();
            self.display = Some(display);
        }
    }
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::Focused(_) => {
                if let Some(display) = &mut self.display {
                    if let Some(gpu) = self.gpu.base().unwrap() {
                        display.ensure_configuration(&gpu).unwrap();
                    }
                }
            }
            WindowEvent::Resized(size) => {
                if let Some(display) = &mut self.display {
                    if let Some(gpu) = self.gpu.base().unwrap() {
                        display.resize(&gpu, size);
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(gpu) = self.gpu.base().unwrap() {
                    gpu.render().surface().unwrap();
                }
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
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

#[derive(Clone)]
pub struct EventsLeaf {
    x: Leaf<f64>,
}

impl Default for EventsLeaf {
    fn default() -> Self {
        Self { x: Leaf::new(0.) }
    }
}
