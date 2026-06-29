use std::rc::Rc;

use glow::HasContext;

pub struct Mesh {
    gl: Rc<glow::Context>,
    vao: glow::NativeVertexArray,
    _vbo: glow::NativeBuffer,
    vertex_count: i32,
}

impl Mesh {
    pub fn new(gl: &Rc<glow::Context>, vertices: &[f32], stride: i32) -> Self {
        let gl = gl.clone();
        let vertex_count = (vertices.len() as i32) / stride;
        unsafe {
            let vao = gl.create_vertex_array().unwrap();
            let vbo = gl.create_buffer().unwrap();

            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, bytemuck::cast_slice(vertices), glow::STATIC_DRAW);

            let stride_bytes = stride * std::mem::size_of::<f32>() as i32;

            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, stride_bytes, 0);

            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, stride_bytes, 3 * std::mem::size_of::<f32>() as i32);

            gl.enable_vertex_attrib_array(2);
            gl.vertex_attrib_pointer_f32(2, 2, glow::FLOAT, false, stride_bytes, 6 * std::mem::size_of::<f32>() as i32);

            gl.bind_vertex_array(None);
            gl.bind_buffer(glow::ARRAY_BUFFER, None);

            Self {
                gl,
                vao,
                _vbo: vbo,
                vertex_count,
            }
        }
    }

    pub fn draw(&self) {
        unsafe {
            self.gl.bind_vertex_array(Some(self.vao));
            self.gl.draw_arrays(glow::TRIANGLES, 0, self.vertex_count);
            self.gl.bind_vertex_array(None);
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_vertex_array(self.vao);
            self.gl.delete_buffer(self._vbo);
        }
    }
}
