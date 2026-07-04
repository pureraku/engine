use engine::assets::asset_manager::MeshType;
use engine::{Engine, EntityId, Vec3, transform::Transform};
use std::rc::Rc;

pub struct Plane {
    pub _id: EntityId,
}

impl Plane {
    pub fn new(engine: &mut Engine) -> Self {
        let mesh = engine.assets().mesh(MeshType::Plane);
        let material = Rc::new(
            engine
                .assets()
                .new_material("basic")
                .with_color(Vec3::new(1.0, 0.0, 0.0)),
        );
        let mut transform = Transform::default();
        transform.position = Vec3::new(0.0, 8.0, 0.0);
        transform.scale = Vec3::new(1.0, 1.0, 1.0);

        Self {
            _id: engine.spawn(mesh, material, transform),
        }
    }
}
