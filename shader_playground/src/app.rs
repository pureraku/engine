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
    pub fn new(engine: &mut Engine) -> Self {
        let mesh = engine.assets().mesh(MeshType::Plane);
        let material = Rc::new(
            engine
                .assets()
                .new_material("basic")
                .with_color(Vec3::new(1.0, 0.0, 0.0)),
        );
        let mut transform = Transform::default();
        transform.position = Vec3::new(0.0, 0.0, 0.0);
        transform.rotation.x = FRAC_PI_2;
        transform.scale = Vec3::new(1.0, 1.0, 1.0);

        Self {
            _id: engine.spawn(mesh, material, transform),
        }
    }
}
