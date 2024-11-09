use graph::*;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

pub type Win = Core;

// #[derive(Default)]
pub struct Core {
    // pub window: Option<Grc<Box<Window>>>,
    // pub window: Leaf<Option<Box<Window>>>,
    pub window: Option<Grc<Window>>,
    events: EventsLeaf,
    resumed: Box<dyn Fn(Grc<Window>)>,
}

impl Core {
    pub fn new(resumed: Box<dyn Fn(Grc<Window>)>) -> Self {
        Self {
            window: None,
            events: EventsLeaf::default(),
            resumed,
        }
    }
}

// impl Default for Core {
//     fn default() -> Self {
//         Self { window: Leaf::new(None), events: EventsLeaf::default() }
//     }
// }

impl ApplicationHandler for Core {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        println!("winit resumed!");
        if self.window.is_none() {
            let window = Grc::new(event_loop
                .create_window(Window::default_attributes())
                .unwrap());
            self.window = Some(window.clone());
            (self.resumed)(window);
        }
    }
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
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
