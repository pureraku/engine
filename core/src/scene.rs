use std::rc::Rc;

use glam::Vec3;

use crate::material::Material;
use crate::mesh::Mesh;
use crate::transform::Transform;

pub struct Object {
    pub transform: Transform,
    on_update: Option<Box<dyn FnMut(&mut Transform, f32, f32)>>,
    mesh: Rc<Mesh>,
    material: Rc<Material>,
}

impl Object {
    fn new(mesh: Rc<Mesh>, material: Rc<Material>, position: Vec3) -> Self {
        let mut transform = Transform::default();
        transform.position = position;
        Self {
            transform,
            on_update: None,
            mesh,
            material,
        }
    }

    pub fn update(&mut self, time: f32, dt: f32) {
        if let Some(f) = &mut self.on_update {
            f(&mut self.transform, time, dt);
        }
    }

    pub fn model_matrix(&self) -> glam::Mat4 {
        self.transform.model_matrix()
    }

    pub fn mesh(&self) -> &Rc<Mesh> {
        &self.mesh
    }

    pub fn material(&self) -> &Rc<Material> {
        &self.material
    }
}

pub struct Scene {
    objects: Vec<Object>,
}

impl Default for Scene {
    fn default() -> Self {
        Self { objects: Vec::new() }
    }
}

impl Scene {
    pub fn update(&mut self, time: f32, dt: f32) {
        for o in &mut self.objects {
            o.update(time, dt);
        }
    }

    pub fn spawn(
        &mut self,
        mesh: Rc<Mesh>,
        material: Rc<Material>,
        position: Vec3,
        on_update: impl FnMut(&mut Transform, f32, f32) + 'static,
    ) {
        let mut obj = Object::new(mesh, material, position);
        obj.on_update = Some(Box::new(on_update));
        self.objects.push(obj);
    }

    pub fn objects(&self) -> &[Object] {
        &self.objects
    }
}
