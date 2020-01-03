extern crate glutin;
extern crate gl;

use glutin::{dpi::LogicalSize, Api, GlRequest, ContextBuilder, GlProfile};
use gl::types::*;
use std::ffi::{CString, c_void};
use std::{ptr, mem};
use std::time::Duration;
use cgmath::{Matrix4, SquareMatrix, Point3, Vector3, perspective, Deg};
use c_str_macro::c_str;

mod shader;
use shader::Shader;

// Vertex data
#[rustfmt::skip]
static VERTEX_DATA: [GLfloat; 9] = [
    0.0, 0.5, 0.0,
    0.5, -0.5, 0.0,
    -0.5, -0.5, 0.0,
];

fn main() {
    let window_width= 640.0;
    let window_height= 480.0;

    let mut events_loop = glutin::EventsLoop::new();
    let window_builder = glutin::WindowBuilder::new()
        .with_title("OpenGL in Rust")
        .with_dimensions(LogicalSize::new(window_width, window_height))
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

    // Create GLSL shaders
    let shader = Shader::new("resources/shaders/shader.vs", "resources/shaders/shader.fs");

    // Matrix
    let model_matrix = Matrix4::identity();
    let view_matrix = Matrix4::look_at(
        Point3 { x: 0.0, y: 0.0, z: 5.0 },
        Point3 { x: 0.0, y: 0.0, z: 0.0 },
        Vector3 { x: 0.0, y: 1.0, z: 0.0 },
    );
    let projection_matrix = perspective(
        Deg(45.0f32),
        window_width as f32 / window_height as f32,
        0.1,
        100.0);

    let mut vao = 0;
    let mut vbo = 0;

    unsafe {
        // Create Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as isize,
            &VERTEX_DATA as *const f32 as *const c_void,
            gl::STATIC_DRAW,
        );

        // Specify the layout of the vertex data
        let pos_attr = gl::GetAttribLocation(shader.id, CString::new("aPosition").unwrap().as_ptr());
        gl::EnableVertexAttribArray(pos_attr as GLuint);
        gl::VertexAttribPointer(
            pos_attr as GLuint,
            3,
            gl::FLOAT,
            gl::FALSE as GLboolean,
            0,
            ptr::null(),
        );

        // unset
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
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
            gl::Viewport(0, 0, window_width as i32, window_height as i32);

            // Clear the screen to black
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // Use shader program
            shader.use_program();
            shader.set_mat4(c_str!("aModel"), &model_matrix);
            shader.set_mat4(c_str!("aView"), &view_matrix);
            shader.set_mat4(c_str!("aProjection"), &projection_matrix);

            // Draw a triangle from the 3 vertices
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.swap_buffers().unwrap();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    // Cleanup
    unsafe {
        shader.delete();
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteVertexArrays(1, &vao);
    }
}
