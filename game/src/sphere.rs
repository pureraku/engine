use engine::assets::MeshType;
use engine::{Engine, EntityId, Vec3, transform::Transform};
use rand::Rng;
use std::rc::Rc;

pub struct Sphere {
    pub id: EntityId,
    base_position: Vec3,
}

impl Sphere {
    pub fn new(engine: &mut Engine, rng: &mut impl Rng) -> Self {
        let mesh = engine.assets().mesh(MeshType::UvSphere {
            stacks: 16,
            slices: 32,
        });

        let material = Rc::new(
            engine
                .assets()
                .new_material("toon")
                .with_color(Vec3::new(0.35, 0.7, 1.0))
                .with_toon_steps(4.0),
        );

        let position = Vec3::new(
            rng.gen_range(-5..=4) as f32,
            rng.gen_range(-5..=4) as f32,
            rng.gen_range(-5..=4) as f32,
        );

        let mut transform = Transform::default();
        transform.position = position;

        Self {
            id: engine.spawn(mesh, material, transform),
            base_position: position,
        }
    }

    pub fn update(&self, engine: &mut Engine, time: f32) {
        let t = engine.transform_mut(self.id);

        t.rotation.x = time * 0.5;
        t.position.y = self.base_position.y + time.sin();
    }
}
