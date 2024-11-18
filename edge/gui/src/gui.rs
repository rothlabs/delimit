use super::*;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, ElementState, WindowEvent};
use winit::event_loop::ActiveEventLoop;

// #[derive(Default)]
pub struct Gui {
    // pub agent: Agent,
    pub agent: agent::Handler,
    pub event: Event,
}

impl Default for Gui {
    fn default() -> Self {
        let (tx, rx) = broadcast::channel::<Grc<Window>>(32);
        let agent = agent::Handler {
            window_send: tx,
            // window_recv: rx,
            agent: Agent::default(),
        };
        spawn(agent.clone().run(rx));
        Self {
            agent,
            event: Event::default(),
        }
    }
}

impl ApplicationHandler for Gui {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.agent.agent.is_empty().unwrap() {
            let fields = Window::default_attributes().with_visible(false);
            let window = event_loop.create_window(fields).unwrap();
            self.agent.window_send.send(window.into()).unwrap();
        }
    }
    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::Resized(size) => {
                self.agent.agent.resize(id, size).unwrap();
            }
            WindowEvent::RedrawRequested => self.agent.agent.render(id).unwrap(),
            WindowEvent::MouseInput { state, .. } => {
                if state == ElementState::Released {
                    let fields = Window::default_attributes().with_visible(false);
                    let window = event_loop.create_window(fields).unwrap();
                    self.agent.window_send.send(window.into()).unwrap();
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
        if let DeviceEvent::MouseMotion { delta } = event {
            let events = self.event.clone();
            tokio::task::spawn(async move {
                let _ = events.x.write(|x| *x = delta.0).await;
            });
        }
    }
}

#[derive(Clone)]
pub struct Event {
    x: Leaf<f64>,
}

impl Default for Event {
    fn default() -> Self {
        Self { x: Leaf::new(0.) }
    }
}

// match event {
//     DeviceEvent::MouseMotion { delta } => {
//         let events = self.event.clone();
//         tokio::task::spawn(async move {
//             let _ = events.x.write(|x| *x = delta.0).await;
//         });
//     }
//     _ => (),
// }
