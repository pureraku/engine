use engine::assets::asset_manager::MeshType;
use engine::{Engine, EntityId, Vec3, transform::Transform};
use std::rc::Rc;

pub struct Ground {
    pub _id: EntityId,
}

impl Ground {
    pub fn new(engine: &mut Engine) -> Self {
        let vertex_shader = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/shaders/ground/ground.vert"
        ))
        .expect("failed to load vertex shader");

        let fragment_shader = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/shaders/ground/ground.frag"
        ))
        .expect("failed to load fragment shader");

        engine
            .assets()
            .create_shader("test", &vertex_shader, &fragment_shader);

        let mesh = engine.assets().mesh(MeshType::Cube);
        let material = Rc::new(engine.assets().new_material("test"));

        let mut transform = Transform::default();
        transform.position = Vec3::new(0.0, -8.0, 0.0);
        transform.scale = Vec3::new(50.0, 1.0, 50.0);

        Self {
            _id: engine.spawn(mesh, material, transform),
        }
    }
}
