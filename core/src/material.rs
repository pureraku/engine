use std::rc::Rc;

use glam::Vec3;

use crate::shader::Shader;
use crate::texture::Texture;

#[derive(Clone)]
pub struct Material {
    pub shader: Rc<Shader>,
    pub albedo: Option<Rc<Texture>>,
    pub base_color: Vec3,
    pub toon_steps: f32,
}

impl Material {
    pub fn new(shader: Rc<Shader>) -> Self {
        Self {
            shader,
            albedo: None,
            base_color: Vec3::ONE,
            toon_steps: 4.0,
        }
    }

    pub fn with_texture(mut self, texture: Rc<Texture>) -> Self {
        self.albedo = Some(texture);
        self
    }

    pub fn with_color(mut self, color: Vec3) -> Self {
        self.base_color = color;
        self
    }

    pub fn with_toon_steps(mut self, steps: f32) -> Self {
        self.toon_steps = steps;
        self
    }
}
