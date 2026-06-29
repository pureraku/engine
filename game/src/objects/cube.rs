use engine::assets::asset_manager::MeshType;
use engine::{Engine, EntityId, Vec3, transform::Transform};
use rand::Rng;
use std::f32::consts::TAU;
use std::rc::Rc;

pub struct Cube {
    pub id: EntityId,

    base_position: Vec3,

    rotation_speed: f32,
    bob_speed: f32,
    bob_height: f32,
    phase: f32,
    rotation_axis: Vec3,
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

            rotation_speed: rng.gen_range(0.5..2.5),
            bob_speed: rng.gen_range(0.5..2.0),
            bob_height: rng.gen_range(0.2..1.0),
            phase: rng.gen_range(0.0..TAU),

            rotation_axis: Vec3::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            )
            .normalize(),
        }
    }

    pub fn update(&self, engine: &mut Engine, time: f32) {
        let t = engine.transform_mut(self.id);

        let angle = time * self.rotation_speed;

        // Independent rotation
        t.rotation = self.rotation_axis * angle;

        // Independent floating
        t.position.y =
            self.base_position.y + self.bob_height * (time * self.bob_speed + self.phase).sin();
    }
}
