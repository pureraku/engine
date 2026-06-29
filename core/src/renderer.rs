use std::rc::Rc;

use glam::{Mat4, Vec3};
use glow::HasContext;

use crate::assets::material::Material;
use crate::camera::Camera;
use crate::scene::Scene;
pub struct Lighting {
    pub light_pos: Vec3,
    pub light_color: Vec3,
    pub light_intensity: f32,
}

impl Default for Lighting {
    fn default() -> Self {
        Self {
            light_pos: Vec3::new(2.0, 2.0, 2.0),
            light_color: Vec3::ONE,
            light_intensity: 1.0,
        }
    }
}

pub struct Renderer {
    gl: Rc<glow::Context>,
}

impl Renderer {
    pub fn new(gl: &Rc<glow::Context>) -> Self {
        unsafe {
            gl.enable(glow::DEPTH_TEST);
        }
        Self { gl: gl.clone() }
    }

    pub fn resize(&self, width: u32, height: u32) {
        unsafe {
            self.gl.viewport(0, 0, width as i32, height.max(1) as i32);
        }
    }

    pub fn begin_frame(&self) {
        unsafe {
            self.gl.clear_color(0.1, 0.1, 0.15, 1.0);
            self.gl
                .clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
    }

    pub fn draw_scene(&self, scene: &Scene, camera: &Camera, lighting: &Lighting) {
        let view = camera.view_matrix();
        let proj = camera.projection_matrix();

        for obj in scene.objects() {
            let mat = obj.material();
            self.draw_object(obj.model_matrix(), mat, view, proj, lighting);
            obj.mesh().draw();
        }
    }

    fn draw_object(
        &self,
        model: Mat4,
        mat: &Material,
        view: Mat4,
        proj: Mat4,
        lighting: &Lighting,
    ) {
        mat.shader.use_program();
        let gl = &self.gl;
        let program = mat.shader.program;

        unsafe {
            set_mat4(gl, program, "model", &model);
            set_mat4(gl, program, "view", &view);
            set_mat4(gl, program, "projection", &proj);
            set_vec3(gl, program, "lightPos", lighting.light_pos);
            set_vec3(gl, program, "lightColor", lighting.light_color);
            set_f32(gl, program, "lightIntensity", lighting.light_intensity);
            set_vec3(gl, program, "baseColor", mat.base_color);
            set_i32(gl, program, "useTexture", mat.albedo.is_some() as i32);
            set_f32(gl, program, "toonSteps", mat.toon_steps);

            if let Some(tex) = &mat.albedo {
                gl.active_texture(glow::TEXTURE0);
                tex.bind();
                set_i32(gl, program, "tex", 0);
            }
        }
    }
}

unsafe fn set_mat4(gl: &glow::Context, program: glow::NativeProgram, name: &str, m: &Mat4) {
    if let Some(loc) = gl.get_uniform_location(program, name) {
        gl.uniform_matrix_4_f32_slice(Some(&loc), false, &m.to_cols_array());
    }
}

unsafe fn set_vec3(gl: &glow::Context, program: glow::NativeProgram, name: &str, v: Vec3) {
    if let Some(loc) = gl.get_uniform_location(program, name) {
        gl.uniform_3_f32(Some(&loc), v.x, v.y, v.z);
    }
}

unsafe fn set_f32(gl: &glow::Context, program: glow::NativeProgram, name: &str, x: f32) {
    if let Some(loc) = gl.get_uniform_location(program, name) {
        gl.uniform_1_f32(Some(&loc), x);
    }
}

unsafe fn set_i32(gl: &glow::Context, program: glow::NativeProgram, name: &str, x: i32) {
    if let Some(loc) = gl.get_uniform_location(program, name) {
        gl.uniform_1_i32(Some(&loc), x);
    }
}
