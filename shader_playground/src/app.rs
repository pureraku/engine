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
        engine.lighting().light_intensity = 1.0;

        self.plane = Some(Plane::new(engine));
        if let Some(plane) = &self.plane {
            let transform = engine.transform_mut(plane.id);
            transform.scale = Vec3::splat(3.0);
            transform.rotation.x = FRAC_PI_2;
        }
    }

    fn update(&mut self, engine: &mut Engine, _time: f32, _dt: f32) {

        if let Some(plane) = &self.plane {
            let transform = engine.transform_mut(plane.id);
            transform.rotation.y +=  0.1;
        }
    }
}

pub struct Plane {
    pub id: EntityId,
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
        );

        let transform = Transform::default();

        Self {
            id: engine.spawn(mesh, material, transform),
        }
    }
}
