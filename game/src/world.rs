use engine::{assets::MeshType, transform::Transform};
use rand::Rng;
use std::rc::Rc;

use engine::{Engine, EntityId, Game, Vec3};

fn random_position(rng: &mut impl Rng) -> Vec3 {
    Vec3::new(
        rng.gen_range(-5..=4) as f32,
        rng.gen_range(-5..=4) as f32,
        rng.gen_range(-5..=4) as f32,
    )
}

pub struct World {
    cubes: Vec<EntityId>,
    spheres: Vec<EntityId>,
    ground: Option<EntityId>,
    sun: Option<EntityId>,
}

impl Default for World {
    fn default() -> Self {
        Self {
            cubes: Vec::new(),
            spheres: Vec::new(),
            ground: None,
            sun: None,
        }
    }
}

impl Game for World {
    fn init(&mut self, engine: &mut Engine) {
        engine.lighting().light_intensity = 4.0;

        let mut rng = rand::thread_rng();

        self.create_sun(engine);
        self.create_cubes(engine, &mut rng);
        self.create_spheres(engine, &mut rng);
        self.create_ground(engine);
    }

    fn update(&mut self, engine: &mut Engine, time: f32, _dt: f32) {
        for id in &self.cubes {
            engine.transform_mut(*id).rotation.y = time;
        }
        for id in &self.spheres {
            let t = engine.transform_mut(*id);

            t.rotation.x = time * 0.5;
            t.position.y += time.sin() * 0.0025;
        }
    }
}

impl World {
    fn create_sun(&mut self, engine: &mut Engine) {
        let sun_mesh = engine.assets().mesh(MeshType::UvSphere {
            stacks: 64,
            slices: 128,
        });

        let sun_mat = Rc::new(
            engine
                .assets()
                .new_material("basic")
                .with_color(Vec3::new(1.0, 1.0, 0.0)),
        );

        let mut transform = Transform::default();
        transform.scale = Vec3::splat(2.0);

        self.sun = Some(engine.spawn(sun_mesh, sun_mat, transform));
    }

    fn create_cubes(&mut self, engine: &mut Engine, rng: &mut impl Rng) {
        let cube_mesh = engine.assets().mesh(MeshType::Cube);

        let cube_mat = Rc::new(
            engine
                .assets()
                .new_material("basic")
                .with_texture(engine.assets().texture("cat", "assets/image.JPG")),
        );

        for _ in 0..8 {
            let mut transform = Transform::default();
            transform.position = random_position(rng);

            let id = engine.spawn(cube_mesh.clone(), cube_mat.clone(), transform);

            self.cubes.push(id);
        }
    }

    fn create_ground(&mut self, engine: &mut Engine) {
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

        let cube_mesh = engine.assets().mesh(MeshType::Cube);
        let cool_mat = Rc::new(engine.assets().new_material("test"));

        let mut transform = Transform::default();
        transform.position = Vec3::new(0.0, -8.0, 0.0);
        transform.scale = Vec3::new(50.0, 1.0, 50.0);

        self.ground = Some(engine.spawn(cube_mesh, cool_mat, transform));
    }

    fn create_spheres(&mut self, engine: &mut Engine, rng: &mut impl Rng) {
        let sphere_mesh = engine.assets().mesh(MeshType::UvSphere {
            stacks: 16,
            slices: 32,
        });

        let sphere_mat = Rc::new(
            engine
                .assets()
                .new_material("toon")
                .with_color(Vec3::new(0.35, 0.7, 1.0))
                .with_toon_steps(4.0),
        );

        for _ in 0..12 {
            let mut transform = Transform::default();
            transform.position = random_position(rng);

            let id = engine.spawn(sphere_mesh.clone(), sphere_mat.clone(), transform);

            self.spheres.push(id);
        }
    }
}
