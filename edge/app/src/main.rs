use graph::*;
use gpu::Gpu;
use win::Win;
use winit::{event_loop::{ControlFlow, EventLoop}, window::Window};

#[tokio::main]
async fn main() {
    let mut win = Win::new(Box::new(wow));
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop.run_app(&mut win).unwrap();
    
}

fn wow(window: Grc<Window>) {
    tokio::task::spawn(async {
        let gpu = Gpu::from_window(window).await;
    });
}

// async fn huh(window: Grc<Window>) -> Res
