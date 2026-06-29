use engine::assets::MeshType;
use rand::Rng;
use std::rc::Rc;

use engine::{Engine, Game, Vec3};
pub struct World;

fn random_position(rng: &mut impl Rng) -> Vec3 {
    Vec3::new(
        rng.gen_range(-5..=4) as f32,
        rng.gen_range(-5..=4) as f32,
        rng.gen_range(-5..=4) as f32,
    )
}

impl Game for World {
    fn init(&mut self, engine: &mut Engine) {
        // Change light intensity
        engine.lighting().light_intensity = 4.0;

        // Create cube mesh and material
        let cube_mesh = engine.assets().mesh(MeshType::Cube);
        let cube_mat = Rc::new(
            engine
                .assets()
                .new_material("basic")
                .with_texture(engine.assets().texture("cat", "assets/image.JPG")),
        );

        // Create sphere mesh and material
        let sphere_mesh = engine.assets().mesh(MeshType::UvSphere {
            stacks: 16,
            slices: 32,
        });
        let sphere_mat = Rc::new(
            engine
                .assets()
                .new_material("toon")
                .with_color(Vec3 {
                    x: 0.35,
                    y: 0.7,
                    z: 1.0,
                })
                .with_toon_steps(4.0),
        );

        // Create ground material
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
        let cool_mat = Rc::new(engine.assets().new_material("test"));

        // Spawn ground
        engine.spawn(
            cube_mesh.clone(),
            cool_mat,
            Vec3::new(0.0, -8.0, 0.0),
            |t, _time, _dt| {
                t.scale = Vec3::new(50.0, 1.0, 50.0);
            },
        );

        let mut rng = rand::thread_rng();

        // Spawn cubes
        for _ in 0..12 {
            engine.spawn(
                cube_mesh.clone(),
                cube_mat.clone(),
                random_position(&mut rng),
                |t, time, _dt| t.rotation.y = time,
            );
        }

        // Spawn spheres
        for _ in 0..8 {
            engine.spawn(
                sphere_mesh.clone(),
                sphere_mat.clone(),
                random_position(&mut rng),
                |t, time, _dt| {
                    t.position.y += time.sin() * 0.0025;
                    t.rotation.x = time * 0.5;
                },
            );
        }
    }
}
