use cgmath::Matrix;
use gl::types::*;
use std::ptr;
use std::ffi::{CString, CStr};
use std::fs::File;
use std::io::Read;
use std::str;

#[allow(dead_code)]
type Vector3 = cgmath::Vector3<f32>;

#[allow(dead_code)]
type Matrix4 = cgmath::Matrix4<f32>;

pub struct Shader {
    pub id: u32,
}

impl Shader {

    pub fn new(vertex_path: &str, fragment_path: &str) -> Shader {
        let vertex_code = Shader::read_file(vertex_path);
        let vertex_shader = Shader::compile(vertex_code.as_str(), gl::VERTEX_SHADER);
        let fragment_code = Shader::read_file(fragment_path);
        let fragment_shader = Shader::compile(fragment_code.as_str(), gl::FRAGMENT_SHADER);

        let id = Shader::link_program(vertex_shader, fragment_shader);

        Shader::delete_shader(vertex_shader);
        Shader::delete_shader(fragment_shader);

        Shader { id }
    }

    pub unsafe fn delete(&self) {
        gl::DeleteProgram(self.id);
    }

    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.id);
    }

    pub unsafe fn set_mat4(&self, name: &CStr, value: &Matrix4) {
        gl::UniformMatrix4fv(
            gl::GetUniformLocation(self.id, name.as_ptr()),
            1,
            gl::FALSE,
            value.as_ptr(),
        );
    }

    fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
        unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, vs);
            gl::AttachShader(program, fs);
            gl::LinkProgram(program);

            // Get the link status
            let mut status = gl::FALSE as GLint;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

            // Fail on error
            if status != (gl::TRUE as GLint) {
                let mut len: GLint = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);

                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1); // subtract 1 to skip trailing null character
                gl::GetProgramInfoLog(
                    program,
                    len,
                    ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );

                panic!(
                    "{}",
                    str::from_utf8(&buf).ok().expect("ProgramInfoLog not valid utf8")
                );
            }

            program
        }
    }

    fn compile(src: &str, ty: GLenum) -> GLuint {
        let shader;
        unsafe {
            shader = gl::CreateShader(ty);

            // Attempt to compile the shader
            let c_str = CString::new(src.as_bytes()).unwrap();
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
            gl::CompileShader(shader);

            // Get the compile status
            let mut status = gl::FALSE as GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

            // Fail on error
            if status != (gl::TRUE as GLint) {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);

                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
                gl::GetShaderInfoLog(
                    shader,
                    len,
                    ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );

                panic!(
                    "{}",
                    str::from_utf8(&buf).ok().expect("ShaderInfoLog not valid utf8")
                );
            }
            shader
        }
    }

    fn delete_shader(shader: GLuint) {
        unsafe {
            gl::DeleteShader(shader);
        }
    }

    fn read_file(path: &str) -> String {
        let mut file = File::open(path)
            .unwrap_or_else(|_| panic!("Failed to open file: {}", path));
        let mut text = String::new();
        file.read_to_string(&mut text)
            .expect("failed to read vertex shader file");

        text
    }
}