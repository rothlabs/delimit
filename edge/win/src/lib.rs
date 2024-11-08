use graph::*;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

pub type Win = Core;

#[derive(Default)]
pub struct Core {
    window: Option<Window>,
    cursor: Cursor,
}

impl ApplicationHandler for Core {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
        }
    }
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                // self.window.as_ref().unwrap().request_redraw();
            },
            // WindowEvent::CursorMoved { device_id, position } => {

            // },
            _ => (),
        }
    }
    fn device_event(
            &mut self,
            event_loop: &ActiveEventLoop,
            device_id: winit::event::DeviceId,
            event: winit::event::DeviceEvent,
        ) {
        match event {
            DeviceEvent::MouseMotion { delta } => {
                // delta.
            }
            _ => ()
        }
    }
}

pub struct Cursor {
    x: Leaf<u32>,
    y: Leaf<u32>,
}

impl Default for Cursor {
    fn default() -> Self {
        Self { 
            x: Leaf::new(0),
            y: Leaf::new(0),
        }
    }
}
