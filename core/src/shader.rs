use std::fs;
use std::rc::Rc;

use glow::HasContext;

pub struct Shader {
    gl: Rc<glow::Context>,
    pub program: glow::NativeProgram,
}

impl Shader {
    pub fn from_files(gl: &Rc<glow::Context>, vertex_path: &str, fragment_path: &str) -> Self {
        let gl = gl.clone();
        let vertex_src = fs::read_to_string(vertex_path).unwrap_or_else(|e| panic!("read {vertex_path}: {e}"));
        let fragment_src = fs::read_to_string(fragment_path).unwrap_or_else(|e| panic!("read {fragment_path}: {e}"));
        Self::from_sources(&gl, &vertex_src, &fragment_src)
    }

    pub fn from_sources(gl: &Rc<glow::Context>, vertex_src: &str, fragment_src: &str) -> Self {
        let gl = gl.clone();
        unsafe {
            let vs = gl.create_shader(glow::VERTEX_SHADER).unwrap();
            gl.shader_source(vs, vertex_src);
            gl.compile_shader(vs);
            if !gl.get_shader_compile_status(vs) {
                eprintln!("vertex shader: {}", gl.get_shader_info_log(vs));
            }

            let fs = gl.create_shader(glow::FRAGMENT_SHADER).unwrap();
            gl.shader_source(fs, fragment_src);
            gl.compile_shader(fs);
            if !gl.get_shader_compile_status(fs) {
                eprintln!("fragment shader: {}", gl.get_shader_info_log(fs));
            }

            let program = gl.create_program().unwrap();
            gl.attach_shader(program, vs);
            gl.attach_shader(program, fs);
            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                eprintln!("program link: {}", gl.get_program_info_log(program));
            }

            gl.delete_shader(vs);
            gl.delete_shader(fs);

            Self { gl, program }
        }
    }

    pub fn use_program(&self) {
        unsafe {
            self.gl.use_program(Some(self.program));
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_program(self.program);
        }
    }
}
