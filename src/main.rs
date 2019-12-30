extern crate glutin;
extern crate gl;

use glutin::{dpi::LogicalSize, Api, GlRequest, ContextBuilder, GlProfile};

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window_builder = glutin::WindowBuilder::new()
        .with_title("OpenGL in Rust")
        .with_dimensions(LogicalSize::new(400.0, 300.0))
        .with_resizable(true);
    let context_builder = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .with_gl_profile(GlProfile::Core)
        .with_vsync(true)
        .with_double_buffer(Some(true));

    let window = context_builder
        .build_windowed(window_builder, &events_loop)
        .unwrap();


    let window = unsafe { window.make_current() }.unwrap();


    unsafe {
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            use glutin::{Event, WindowEvent};

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

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.swap_buffers().unwrap();
    }
}
