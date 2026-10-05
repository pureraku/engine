use crate::arena::Arena;
use crate::particles::ParticleSystem;
use engine::assets::asset_manager::MeshType;
use engine::{Engine, EntityId, Vec3, transform::Transform};
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjectileOwner {
    Player,
    Enemy,
}

pub struct Projectile {
    pub entity: EntityId,
    pub owner: ProjectileOwner,
    pub position: Vec3,
    pub velocity: Vec3,
    pub damage: f32,
    pub is_rocket: bool,
    pub splash_radius: f32,
    pub splash_damage: f32,
    pub life: f32,
    pub color: Vec3,
}

pub struct HitEvent {
    pub target_enemy_idx: Option<usize>,
    pub hit_player: bool,
    pub damage: f32,
    pub hit_pos: Vec3,
    pub is_rocket: bool,
    pub splash_radius: f32,
    pub splash_damage: f32,
}

pub struct ProjectileManager {
    projectiles: Vec<Projectile>,
}

impl ProjectileManager {
    pub fn new() -> Self {
        Self {
            projectiles: Vec::new(),
        }
    }

    pub fn spawn(
        &mut self,
        engine: &mut Engine,
        owner: ProjectileOwner,
        origin: Vec3,
        direction: Vec3,
        speed: f32,
        damage: f32,
        is_rocket: bool,
        splash_radius: f32,
        splash_damage: f32,
        color: Vec3,
    ) {
        let cube_mesh = engine.assets().mesh(MeshType::Cube);
        let mat = Rc::new(engine.assets().new_material("glow").with_color(color));

        let mut tr = Transform::default();
        tr.position = origin;
        tr.scale = if is_rocket {
            Vec3::new(0.3, 0.3, 0.8)
        } else {
            Vec3::new(0.12, 0.12, 0.45)
        };

        let entity = engine.spawn(cube_mesh, mat, tr);

        self.projectiles.push(Projectile {
            entity,
            owner,
            position: origin,
            velocity: direction.normalize() * speed,
            damage,
            is_rocket,
            splash_radius,
            splash_damage,
            life: 3.5,
            color,
        });
    }

    pub fn update(
        &mut self,
        engine: &mut Engine,
        arena: &Arena,
        particles: &mut ParticleSystem,
        player_pos: Vec3,
        player_radius: f32,
        enemy_targets: &[(usize, Vec3, f32)], // (index, position, radius)
        dt: f32,
    ) -> Vec<HitEvent> {
        let mut hit_events = Vec::new();
        let mut to_remove = Vec::new();

        for (i, p) in self.projectiles.iter_mut().enumerate() {
            p.life -= dt;
            if p.life <= 0.0 {
                to_remove.push(i);
                continue;
            }

            let step = p.velocity * dt;
            let step_dist = step.length();
            let dir = if step_dist > 0.0001 { step / step_dist } else { Vec3::Z };
            let next_pos = p.position + step;

            let mut collided = false;
            let mut hit_point;

            // 1. Raycast collision with static arena geometry
            if let Some(hit_t) = arena.raycast(p.position, dir, step_dist) {
                collided = true;
                hit_point = p.position + dir * hit_t;

                if p.is_rocket {
                    particles.spawn_explosion(engine, hit_point, p.splash_radius);
                    hit_events.push(HitEvent {
                        target_enemy_idx: None,
                        hit_player: false,
                        damage: 0.0,
                        hit_pos: hit_point,
                        is_rocket: true,
                        splash_radius: p.splash_radius,
                        splash_damage: p.splash_damage,
                    });
                } else {
                    particles.spawn_spark(engine, hit_point, -dir, p.color);
                }
            }

            // 2. Collision with enemies (if player projectile)
            if !collided && p.owner == ProjectileOwner::Player {
                for &(enemy_idx, e_pos, e_rad) in enemy_targets {
                    let d = (next_pos - e_pos).length();
                    if d <= e_rad + 0.3 {
                        collided = true;
                        hit_point = next_pos;

                        if p.is_rocket {
                            particles.spawn_explosion(engine, hit_point, p.splash_radius);
                            hit_events.push(HitEvent {
                                target_enemy_idx: Some(enemy_idx),
                                hit_player: false,
                                damage: p.damage,
                                hit_pos: hit_point,
                                is_rocket: true,
                                splash_radius: p.splash_radius,
                                splash_damage: p.splash_damage,
                            });
                        } else {
                            particles.spawn_spark(engine, hit_point, -dir, p.color);
                            hit_events.push(HitEvent {
                                target_enemy_idx: Some(enemy_idx),
                                hit_player: false,
                                damage: p.damage,
                                hit_pos: hit_point,
                                is_rocket: false,
                                splash_radius: 0.0,
                                splash_damage: 0.0,
                            });
                        }
                        break;
                    }
                }
            }

            // 3. Collision with player (if enemy projectile)
            if !collided && p.owner == ProjectileOwner::Enemy {
                let d = (next_pos - (player_pos + Vec3::new(0.0, 1.0, 0.0))).length();
                if d <= player_radius + 0.4 {
                    collided = true;
                    hit_point = next_pos;

                    if p.is_rocket {
                        particles.spawn_explosion(engine, hit_point, p.splash_radius);
                        hit_events.push(HitEvent {
                            target_enemy_idx: None,
                            hit_player: true,
                            damage: p.damage,
                            hit_pos: hit_point,
                            is_rocket: true,
                            splash_radius: p.splash_radius,
                            splash_damage: p.splash_damage,
                        });
                    } else {
                        particles.spawn_spark(engine, hit_point, -dir, p.color);
                        hit_events.push(HitEvent {
                            target_enemy_idx: None,
                            hit_player: true,
                            damage: p.damage,
                            hit_pos: hit_point,
                            is_rocket: false,
                            splash_radius: 0.0,
                            splash_damage: 0.0,
                        });
                    }
                }
            }

            if collided {
                to_remove.push(i);
            } else {
                p.position = next_pos;
                let tr = engine.transform_mut(p.entity);
                tr.position = p.position;
            }
        }

        // Clean up collided/expired projectiles
        for &idx in to_remove.iter().rev() {
            let p = self.projectiles.remove(idx);
            engine.despawn(p.entity);
        }

        hit_events
    }

    pub fn clear(&mut self, engine: &mut Engine) {
        for p in self.projectiles.drain(..) {
            engine.despawn(p.entity);
        }
    }
}
