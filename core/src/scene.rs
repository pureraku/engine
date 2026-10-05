use std::rc::Rc;

use crate::assets::material::Material;
use crate::assets::mesh::Mesh;
use crate::transform::Transform;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EntityId(pub usize);

pub struct Object {
    pub transform: Transform,
    pub visible: bool,
    mesh: Rc<Mesh>,
    material: Rc<Material>,
}

impl Object {
    fn new(mesh: Rc<Mesh>, material: Rc<Material>, transform: Transform) -> Self {
        Self {
            transform,
            visible: true,
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

    pub fn set_mesh(&mut self, mesh: Rc<Mesh>) {
        self.mesh = mesh;
    }

    pub fn material(&self) -> &Rc<Material> {
        &self.material
    }

    pub fn set_material(&mut self, material: Rc<Material>) {
        self.material = material;
    }
}

#[derive(Default)]
pub struct Scene {
    objects: Vec<Option<Object>>,
}

impl Scene {
    pub fn spawn(
        &mut self,
        mesh: Rc<Mesh>,
        material: Rc<Material>,
        transform: Transform,
    ) -> EntityId {
        let obj = Object::new(mesh, material, transform);
        for (i, slot) in self.objects.iter_mut().enumerate() {
            if slot.is_none() {
                *slot = Some(obj);
                return EntityId(i);
            }
        }
        self.objects.push(Some(obj));
        EntityId(self.objects.len() - 1)
    }

    pub fn despawn(&mut self, id: EntityId) {
        if id.0 < self.objects.len() {
            self.objects[id.0] = None;
        }
    }

    pub fn is_alive(&self, id: EntityId) -> bool {
        self.objects.get(id.0).and_then(|opt| opt.as_ref()).is_some()
    }

    pub fn object_mut(&mut self, id: EntityId) -> &mut Object {
        self.objects[id.0]
            .as_mut()
            .expect("Entity is despawned or does not exist")
    }

    pub fn object(&self, id: EntityId) -> Option<&Object> {
        self.objects.get(id.0).and_then(|opt| opt.as_ref())
    }

    pub fn set_visible(&mut self, id: EntityId, visible: bool) {
        if let Some(Some(obj)) = self.objects.get_mut(id.0) {
            obj.visible = visible;
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn objects(&self) -> impl Iterator<Item = &Object> {
        self.objects.iter().filter_map(|opt| opt.as_ref())
    }
}
