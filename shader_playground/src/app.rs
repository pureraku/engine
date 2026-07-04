use engine::assets::asset_manager::MeshType;
use engine::{Engine, EntityId, Game, Vec3, transform::Transform};
use std::f32::consts::FRAC_PI_2;
use std::rc::Rc;

pub struct App {
    plane: Option<Plane>,
}

impl Default for App {
    fn default() -> Self {
        Self { plane: None }
    }
}

impl Game for App {
    fn init(&mut self, engine: &mut Engine) {
        engine.lighting().light_intensity = 4.0;

        self.plane = Some(Plane::new(engine));
    }

    fn update(&mut self, _engine: &mut Engine, _time: f32, _dt: f32) {}
}

pub struct Plane {
    pub _id: EntityId,
}

impl Plane {
    fn load_shaders() -> (String, String) {
        let vertex =
            std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/shader/plane.vert"))
                .expect("failed to load vertex shader");

        let fragment =
            std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/shader/plane.frag"))
                .expect("failed to load fragment shader");

        (vertex, fragment)
    }

    pub fn new(engine: &mut Engine) -> Self {
        let (vertex_shader, fragment_shader) = Self::load_shaders();

        engine
            .assets()
            .create_shader("test", &vertex_shader, &fragment_shader);

        let mesh = engine.assets().mesh(MeshType::Plane);

        let material = Rc::new(
            engine
                .assets()
                .new_material("test")
                .with_color(Vec3::new(1.0, 0.0, 0.0)),
        );

        let mut transform = Transform::default();
        transform.scale = Vec3::splat(3.0);
        transform.rotation.x = FRAC_PI_2;

        Self {
            _id: engine.spawn(mesh, material, transform),
        }
    }
}
