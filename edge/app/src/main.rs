use winit::event_loop::EventLoop;

#[tokio::main]
async fn main() {
    let mut app = win::App::default();
    let event_loop = EventLoop::new().unwrap();
    event_loop.run_app(&mut app).unwrap();
}

// struct Start {

// }
