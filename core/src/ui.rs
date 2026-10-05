use glam::{Mat4, Vec4};
use glow::HasContext;
use std::rc::Rc;

const VERT_SRC: &str = r#"#version 330 core
layout (location = 0) in vec2 aPos;
layout (location = 1) in vec4 aColor;

uniform mat4 projection;
out vec4 vColor;

void main() {
    gl_Position = projection * vec4(aPos, 0.0, 1.0);
    vColor = aColor;
}
"#;

const FRAG_SRC: &str = r#"#version 330 core
in vec4 vColor;
out vec4 FragColor;

void main() {
    FragColor = vColor;
}
"#;

pub struct UiRenderer {
    gl: Rc<glow::Context>,
    program: glow::NativeProgram,
    vao: glow::NativeVertexArray,
    vbo: glow::NativeBuffer,
    vertex_buffer: Vec<f32>,
    vertex_count: i32,
    pub screen_width: f32,
    pub screen_height: f32,
}

impl UiRenderer {
    pub fn new(gl: &Rc<glow::Context>, width: f32, height: f32) -> Self {
        let gl = gl.clone();
        unsafe {
            let vs = gl.create_shader(glow::VERTEX_SHADER).unwrap();
            gl.shader_source(vs, VERT_SRC);
            gl.compile_shader(vs);

            let fs = gl.create_shader(glow::FRAGMENT_SHADER).unwrap();
            gl.shader_source(fs, FRAG_SRC);
            gl.compile_shader(fs);

            let program = gl.create_program().unwrap();
            gl.attach_shader(program, vs);
            gl.attach_shader(program, fs);
            gl.link_program(program);

            gl.delete_shader(vs);
            gl.delete_shader(fs);

            let vao = gl.create_vertex_array().unwrap();
            let vbo = gl.create_buffer().unwrap();

            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

            // stride = 6 floats (x, y, r, g, b, a) = 24 bytes
            let stride = 6 * std::mem::size_of::<f32>() as i32;
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, stride, 0);

            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(1, 4, glow::FLOAT, false, stride, 2 * std::mem::size_of::<f32>() as i32);

            gl.bind_vertex_array(None);
            gl.bind_buffer(glow::ARRAY_BUFFER, None);

            Self {
                gl,
                program,
                vao,
                vbo,
                vertex_buffer: Vec::with_capacity(4096),
                vertex_count: 0,
                screen_width: width,
                screen_height: height,
            }
        }
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.screen_width = width;
        self.screen_height = height;
    }

    pub fn begin(&mut self) {
        self.vertex_buffer.clear();
        self.vertex_count = 0;
    }

    fn push_vert(&mut self, x: f32, y: f32, c: Vec4) {
        self.vertex_buffer.extend_from_slice(&[x, y, c.x, c.y, c.z, c.w]);
        self.vertex_count += 1;
    }

    pub fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32, c: Vec4) {
        self.push_vert(x, y, c);
        self.push_vert(x + w, y, c);
        self.push_vert(x + w, y + h, c);

        self.push_vert(x, y, c);
        self.push_vert(x + w, y + h, c);
        self.push_vert(x, y + h, c);
    }

    pub fn draw_rect_outline(&mut self, x: f32, y: f32, w: f32, h: f32, thickness: f32, c: Vec4) {
        // Top
        self.draw_rect(x, y, w, thickness, c);
        // Bottom
        self.draw_rect(x, y + h - thickness, w, thickness, c);
        // Left
        self.draw_rect(x, y, thickness, h, c);
        // Right
        self.draw_rect(x + w - thickness, y, thickness, h, c);
    }

    pub fn draw_crosshair(&mut self, cx: f32, cy: f32, size: f32, gap: f32, thickness: f32, c: Vec4) {
        let half_t = thickness * 0.5;
        // Top
        self.draw_rect(cx - half_t, cy - gap - size, thickness, size, c);
        // Bottom
        self.draw_rect(cx - half_t, cy + gap, thickness, size, c);
        // Left
        self.draw_rect(cx - gap - size, cy - half_t, size, thickness, c);
        // Right
        self.draw_rect(cx + gap, cy - half_t, size, thickness, c);
        // Center tiny dot
        self.draw_rect(cx - 1.0, cy - 1.0, 2.0, 2.0, c);
    }

    pub fn draw_hitmarker(&mut self, cx: f32, cy: f32, size: f32, thickness: f32, c: Vec4) {
        let d = size * 0.707;
        let t = thickness;
        // 4 diagonal ticks
        self.draw_rect(cx + 4.0, cy + 4.0, d, t, c);
        self.draw_rect(cx - 4.0 - d, cy + 4.0, d, t, c);
        self.draw_rect(cx + 4.0, cy - 4.0 - t, d, t, c);
        self.draw_rect(cx - 4.0 - d, cy - 4.0 - t, d, t, c);
    }

    pub fn draw_bar(&mut self, x: f32, y: f32, w: f32, h: f32, current: f32, max: f32, fg: Vec4, bg: Vec4, border: Vec4) {
        // Outline
        self.draw_rect_outline(x, y, w, h, 2.0, border);
        // Background
        self.draw_rect(x + 2.0, y + 2.0, w - 4.0, h - 4.0, bg);
        // Foreground fill
        let ratio = (current / max.max(1.0)).clamp(0.0, 1.0);
        let fill_w = (w - 4.0) * ratio;
        if fill_w > 0.0 {
            self.draw_rect(x + 2.0, y + 2.0, fill_w, h - 4.0, fg);
        }
    }

    pub fn draw_char(&mut self, x: f32, y: f32, ch: char, scale: f32, c: Vec4) -> f32 {
        let glyph = get_glyph(ch);
        let pixel_size = scale;
        for col in 0..5 {
            let col_byte = glyph[col];
            for row in 0..7 {
                if (col_byte & (1 << row)) != 0 {
                    self.draw_rect(
                        x + (col as f32) * pixel_size,
                        y + (row as f32) * pixel_size,
                        pixel_size,
                        pixel_size,
                        c,
                    );
                }
            }
        }
        6.0 * scale
    }

    pub fn draw_text(&mut self, mut x: f32, y: f32, text: &str, scale: f32, c: Vec4) {
        for ch in text.chars() {
            let adv = self.draw_char(x, y, ch, scale, c);
            x += adv;
        }
    }

    pub fn flush(&mut self) {
        if self.vertex_count == 0 {
            return;
        }

        unsafe {
            self.gl.disable(glow::DEPTH_TEST);
            self.gl.enable(glow::BLEND);
            self.gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);

            self.gl.use_program(Some(self.program));

            let proj = Mat4::orthographic_rh_gl(0.0, self.screen_width, self.screen_height, 0.0, -1.0, 1.0);
            if let Some(loc) = self.gl.get_uniform_location(self.program, "projection") {
                self.gl.uniform_matrix_4_f32_slice(Some(&loc), false, &proj.to_cols_array());
            }

            self.gl.bind_vertex_array(Some(self.vao));
            self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
            self.gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&self.vertex_buffer),
                glow::STREAM_DRAW,
            );

            self.gl.draw_arrays(glow::TRIANGLES, 0, self.vertex_count);

            self.gl.bind_vertex_array(None);
            self.gl.bind_buffer(glow::ARRAY_BUFFER, None);
            self.gl.use_program(None);

            self.gl.enable(glow::DEPTH_TEST);
        }

        self.vertex_buffer.clear();
        self.vertex_count = 0;
    }
}

impl Drop for UiRenderer {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_program(self.program);
            self.gl.delete_vertex_array(self.vao);
            self.gl.delete_buffer(self.vbo);
        }
    }
}

fn get_glyph(c: char) -> [u8; 5] {
    let uc = c.to_ascii_uppercase();
    match uc {
        '0' => [0x3E, 0x51, 0x49, 0x45, 0x3E],
        '1' => [0x00, 0x42, 0x7F, 0x40, 0x00],
        '2' => [0x42, 0x61, 0x51, 0x49, 0x46],
        '3' => [0x21, 0x41, 0x45, 0x4B, 0x31],
        '4' => [0x18, 0x14, 0x12, 0x7F, 0x10],
        '5' => [0x27, 0x45, 0x45, 0x45, 0x39],
        '6' => [0x3C, 0x4A, 0x49, 0x49, 0x30],
        '7' => [0x01, 0x71, 0x09, 0x05, 0x03],
        '8' => [0x36, 0x49, 0x49, 0x49, 0x36],
        '9' => [0x06, 0x49, 0x49, 0x29, 0x1E],
        'A' => [0x7E, 0x11, 0x11, 0x11, 0x7E],
        'B' => [0x7F, 0x49, 0x49, 0x49, 0x36],
        'C' => [0x3E, 0x41, 0x41, 0x41, 0x22],
        'D' => [0x7F, 0x41, 0x41, 0x22, 0x1C],
        'E' => [0x7F, 0x49, 0x49, 0x49, 0x41],
        'F' => [0x7F, 0x09, 0x09, 0x09, 0x01],
        'G' => [0x3E, 0x41, 0x49, 0x49, 0x7A],
        'H' => [0x7F, 0x08, 0x08, 0x08, 0x7F],
        'I' => [0x00, 0x41, 0x7F, 0x41, 0x00],
        'J' => [0x20, 0x40, 0x41, 0x3F, 0x01],
        'K' => [0x7F, 0x08, 0x14, 0x22, 0x41],
        'L' => [0x7F, 0x40, 0x40, 0x40, 0x40],
        'M' => [0x7F, 0x02, 0x0C, 0x02, 0x7F],
        'N' => [0x7F, 0x04, 0x08, 0x10, 0x7F],
        'O' => [0x3E, 0x41, 0x41, 0x41, 0x3E],
        'P' => [0x7F, 0x09, 0x09, 0x09, 0x06],
        'Q' => [0x3E, 0x41, 0x51, 0x21, 0x5E],
        'R' => [0x7F, 0x09, 0x19, 0x29, 0x46],
        'S' => [0x46, 0x49, 0x49, 0x49, 0x31],
        'T' => [0x01, 0x01, 0x7F, 0x01, 0x01],
        'U' => [0x3F, 0x40, 0x40, 0x40, 0x3F],
        'V' => [0x1F, 0x20, 0x40, 0x20, 0x1F],
        'W' => [0x7F, 0x20, 0x18, 0x20, 0x7F],
        'X' => [0x63, 0x14, 0x08, 0x14, 0x63],
        'Y' => [0x07, 0x08, 0x70, 0x08, 0x07],
        'Z' => [0x61, 0x51, 0x49, 0x45, 0x43],
        ' ' => [0x00, 0x00, 0x00, 0x00, 0x00],
        ':' => [0x00, 0x36, 0x36, 0x00, 0x00],
        '-' => [0x08, 0x08, 0x08, 0x08, 0x08],
        '+' => [0x08, 0x08, 0x3E, 0x08, 0x08],
        '!' => [0x00, 0x00, 0x5F, 0x00, 0x00],
        '/' => [0x20, 0x10, 0x08, 0x04, 0x02],
        '.' => [0x00, 0x60, 0x60, 0x00, 0x00],
        '%' => [0x23, 0x13, 0x08, 0x64, 0x62],
        '[' => [0x00, 0x7F, 0x41, 0x41, 0x00],
        ']' => [0x00, 0x41, 0x41, 0x7F, 0x00],
        '<' => [0x08, 0x14, 0x22, 0x41, 0x00],
        '>' => [0x00, 0x41, 0x22, 0x14, 0x08],
        _ => [0x7F, 0x7F, 0x7F, 0x7F, 0x7F],
    }
}
