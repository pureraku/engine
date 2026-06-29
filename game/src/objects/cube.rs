use engine::assets::asset_manager::MeshType;
use engine::{Engine, EntityId, Vec3, transform::Transform};
use rand::Rng;
use std::rc::Rc;

pub struct Cube {
    pub id: EntityId,
}

impl Cube {
    pub fn new(engine: &mut Engine, rng: &mut impl Rng) -> Self {
        let mesh = engine.assets().mesh(MeshType::Cube);

        let material = Rc::new(
            engine
                .assets()
                .new_material("basic")
                .with_texture(engine.assets().texture("cat", "assets/cat.jpg")),
        );

        let mut transform = Transform::default();
        transform.position = Vec3::new(
            rng.gen_range(-5..=4) as f32,
            rng.gen_range(-5..=4) as f32,
            rng.gen_range(-5..=4) as f32,
        );

        Self {
            id: engine.spawn(mesh, material, transform),
        }
    }

    pub fn update(&self, engine: &mut Engine, time: f32) {
        engine.transform_mut(self.id).rotation.y = time;
    }
}
