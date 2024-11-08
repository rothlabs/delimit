use win::Win;
use winit::event_loop::{ControlFlow, EventLoop};

fn main() {
    let mut win = Win::default();
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop.run_app(&mut win).unwrap();
}
