use super::*;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, WindowEvent};
use winit::event_loop::ActiveEventLoop;

#[derive(Default)]
pub struct App {
    events: EventsLeaf,
    displays: Displays,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.displays.is_empty().unwrap() {
            let fields = Window::default_attributes().with_visible(false);
            let window = Grc::new(event_loop.create_window(fields).unwrap());
            spawn(self.displays.clone().main(window));
        }
    }
    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::Resized(size) => {
                self.displays.resize(id, size).unwrap();
            }
            WindowEvent::RedrawRequested => self.displays.render(id).unwrap(),
            WindowEvent::MouseInput { device_id, state, button } => {
                let fields = Window::default_attributes().with_visible(false);
                let window = Grc::new(event_loop.create_window(fields).unwrap());
                spawn(self.displays.clone().display(window));
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
