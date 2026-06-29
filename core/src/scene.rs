use std::rc::Rc;

use crate::material::Material;
use crate::mesh::Mesh;
use crate::transform::Transform;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EntityId(pub usize);

pub struct Object {
    pub transform: Transform,
    mesh: Rc<Mesh>,
    material: Rc<Material>,
}

impl Object {
    fn new(mesh: Rc<Mesh>, material: Rc<Material>, transform: Transform) -> Self {
        Self {
            transform,
            mesh,
            material,
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

#[derive(Default)]
pub struct Scene {
    objects: Vec<Object>,
}

impl Scene {
    pub fn spawn(
        &mut self,
        mesh: Rc<Mesh>,
        material: Rc<Material>,
        transform: Transform,
    ) -> EntityId {
        let obj = Object::new(mesh, material, transform);
        self.objects.push(obj);

        EntityId(self.objects.len() - 1)
    }

    pub fn object_mut(&mut self, id: EntityId) -> &mut Object {
        &mut self.objects[id.0]
    }

    pub fn objects(&self) -> &[Object] {
        &self.objects
    }
}
