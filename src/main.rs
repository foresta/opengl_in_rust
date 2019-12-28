extern crate winit;

use winit::{Event, WindowEvent};
use winit::dpi::LogicalSize;

fn main() {
    let mut events_loop = winit::EventsLoop::new();
    let window = winit::WindowBuilder::new()
        .with_title("OpenGL in Rust")
        .build(&events_loop)
        .unwrap();

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            match event {
                Event::WindowEvent {
                    event: WindowEvent::Resized(LogicalSize { width, height }),
                    ..
                } => {
                    println!("The window was resized to {}x{}", width, height);
                },
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    running = false;
                }
                _ => ()
            }
        });
    }
}
