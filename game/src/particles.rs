use engine::assets::asset_manager::MeshType;
use engine::{Engine, EntityId, Vec3, transform::Transform};
use rand::Rng;
use std::rc::Rc;

#[derive(Clone, Copy)]
pub struct Particle {
    pub entity: EntityId,
    pub active: bool,
    pub position: Vec3,
    pub velocity: Vec3,
    pub life: f32,
    pub max_life: f32,
    pub initial_scale: f32,
    pub gravity: f32,
}

#[allow(dead_code)]
pub struct ParticleSystem {
    particles: Vec<Particle>,
    next_index: usize,
    spark_material: Rc<engine::assets::material::Material>,
    explosion_material: Rc<engine::assets::material::Material>,
}

impl ParticleSystem {
    pub fn new(engine: &mut Engine, max_particles: usize) -> Self {
        let cube_mesh = engine.assets().mesh(MeshType::Cube);
        let spark_material = Rc::new(
            engine.assets().new_material("glow").with_color(Vec3::new(1.0, 0.8, 0.2)),
        );
        let explosion_material = Rc::new(
            engine.assets().new_material("glow").with_color(Vec3::new(1.0, 0.3, 0.1)),
        );

        let mut particles = Vec::with_capacity(max_particles);
        for _ in 0..max_particles {
            let entity = engine.spawn(cube_mesh.clone(), spark_material.clone(), Transform::default());
            engine.set_visible(entity, false);

            particles.push(Particle {
                entity,
                active: false,
                position: Vec3::ZERO,
                velocity: Vec3::ZERO,
                life: 0.0,
                max_life: 1.0,
                initial_scale: 0.1,
                gravity: 15.0,
            });
        }

        Self {
            particles,
            next_index: 0,
            spark_material,
            explosion_material,
        }
    }

    pub fn spawn_spark(&mut self, engine: &mut Engine, pos: Vec3, normal: Vec3, color: Vec3) {
        let mut rng = rand::thread_rng();
        let count = rng.gen_range(6..12);

        for _ in 0..count {
            let idx = self.next_index;
            self.next_index = (self.next_index + 1) % self.particles.len();

            let p = &mut self.particles[idx];
            p.active = true;
            p.position = pos;

            // Random reflection direction
            let rx = rng.gen_range(-1.0..1.0);
            let ry = rng.gen_range(-1.0..1.0);
            let rz = rng.gen_range(-1.0..1.0);
            let rand_dir = (normal * 1.5 + Vec3::new(rx, ry, rz)).normalize();

            let speed = rng.gen_range(6.0..18.0);
            p.velocity = rand_dir * speed;
            p.max_life = rng.gen_range(0.2..0.5);
            p.life = p.max_life;
            p.initial_scale = rng.gen_range(0.08..0.16);
            p.gravity = 18.0;

            let mat = Rc::new(engine.assets().new_material("glow").with_color(color));
            engine.object_mut(p.entity).set_material(mat);
            engine.set_visible(p.entity, true);
        }
    }

    pub fn spawn_explosion(&mut self, engine: &mut Engine, pos: Vec3, radius: f32) {
        let mut rng = rand::thread_rng();
        let count = 28;

        for _ in 0..count {
            let idx = self.next_index;
            self.next_index = (self.next_index + 1) % self.particles.len();

            let p = &mut self.particles[idx];
            p.active = true;
            p.position = pos + Vec3::new(
                rng.gen_range(-0.5..0.5),
                rng.gen_range(-0.5..0.5),
                rng.gen_range(-0.5..0.5),
            );

            let rx = rng.gen_range(-1.0..1.0);
            let ry = rng.gen_range(0.2..1.5);
            let rz = rng.gen_range(-1.0..1.0);
            let dir = Vec3::new(rx, ry, rz).normalize();

            let speed = rng.gen_range(8.0..26.0) * (radius / 5.0);
            p.velocity = dir * speed;
            p.max_life = rng.gen_range(0.4..0.8);
            p.life = p.max_life;
            p.initial_scale = rng.gen_range(0.25..0.6);
            p.gravity = 12.0;

            let col = if rng.gen_bool(0.5) {
                Vec3::new(1.0, 0.4, 0.05)
            } else {
                Vec3::new(1.0, 0.1, 0.8)
            };
            let mat = Rc::new(engine.assets().new_material("glow").with_color(col));
            engine.object_mut(p.entity).set_material(mat);
            engine.set_visible(p.entity, true);
        }
    }

    pub fn spawn_debris(&mut self, engine: &mut Engine, pos: Vec3, color: Vec3) {
        let mut rng = rand::thread_rng();
        for _ in 0..14 {
            let idx = self.next_index;
            self.next_index = (self.next_index + 1) % self.particles.len();

            let p = &mut self.particles[idx];
            p.active = true;
            p.position = pos;

            let rx = rng.gen_range(-1.0..1.0);
            let ry = rng.gen_range(0.5..2.0);
            let rz = rng.gen_range(-1.0..1.0);
            let dir = Vec3::new(rx, ry, rz).normalize();

            p.velocity = dir * rng.gen_range(5.0..16.0);
            p.max_life = rng.gen_range(0.6..1.2);
            p.life = p.max_life;
            p.initial_scale = rng.gen_range(0.18..0.35);
            p.gravity = 22.0;

            let mat = Rc::new(engine.assets().new_material("basic").with_color(color));
            engine.object_mut(p.entity).set_material(mat);
            engine.set_visible(p.entity, true);
        }
    }

    pub fn update(&mut self, engine: &mut Engine, dt: f32) {
        for p in &mut self.particles {
            if !p.active {
                continue;
            }

            p.life -= dt;
            if p.life <= 0.0 {
                p.active = false;
                engine.set_visible(p.entity, false);
                continue;
            }

            p.velocity.y -= p.gravity * dt;
            p.position += p.velocity * dt;

            // Bounce on ground
            if p.position.y <= -7.0 {
                p.position.y = -7.0;
                p.velocity.y = -p.velocity.y * 0.4;
                p.velocity.x *= 0.7;
                p.velocity.z *= 0.7;
            }

            let progress = p.life / p.max_life;
            let current_scale = p.initial_scale * progress;

            let tr = engine.transform_mut(p.entity);
            tr.position = p.position;
            tr.scale = Vec3::splat(current_scale);
        }
    }
}
