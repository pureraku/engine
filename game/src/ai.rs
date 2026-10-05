use crate::arena::Arena;
use crate::particles::ParticleSystem;
use crate::projectiles::{ProjectileManager, ProjectileOwner};
use engine::assets::asset_manager::MeshType;
use engine::{Engine, EntityId, Vec3, transform::Transform};
use rand::Rng;
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnemyType {
    Drone,
    Trooper,
    Titan,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum EnemyState {
    Patrol,
    Pursuit,
    Attack,
    TakeCover,
}

#[allow(dead_code)]
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub position: Vec3,
    pub velocity: Vec3,
    pub yaw: f32,

    pub health: f32,
    pub max_health: f32,
    pub radius: f32,
    pub height: f32,
    pub speed: f32,

    pub state: EnemyState,
    pub state_timer: f32,
    pub attack_cooldown: f32,
    pub burst_count: i32,
    pub burst_timer: f32,

    pub hit_flash_timer: f32,
    pub strafe_dir: f32,
    pub score_value: u32,

    // Visual entities
    pub main_entity: EntityId,
    pub eye_entity: EntityId,
    pub extra_entity: Option<EntityId>,

    default_mat: Rc<engine::assets::material::Material>,
    flash_mat: Rc<engine::assets::material::Material>,
}

pub struct AiManager {
    pub enemies: Vec<Enemy>,
    pub total_kills: u32,
    pub score: u32,
    pub current_wave: u32,
    pub wave_cleared: bool,
    pub wave_intermission_timer: f32,
    cube_mesh: Rc<engine::assets::mesh::Mesh>,
}

impl AiManager {
    pub fn new(engine: &mut Engine) -> Self {
        let cube_mesh = engine.assets().mesh(MeshType::Cube);
        Self {
            enemies: Vec::new(),
            total_kills: 0,
            score: 0,
            current_wave: 1,
            wave_cleared: false,
            wave_intermission_timer: 2.0,
            cube_mesh,
        }
    }

    pub fn spawn_enemy(&mut self, engine: &mut Engine, enemy_type: EnemyType, spawn_pos: Vec3) {
        let mut rng = rand::thread_rng();

        let (health, radius, height, speed, score_value, main_scale, main_color, eye_color) = match enemy_type {
            EnemyType::Drone => (
                40.0,
                0.7,
                1.4,
                10.0,
                100,
                Vec3::new(0.8, 0.8, 0.8),
                Vec3::new(0.25, 0.28, 0.35),
                Vec3::new(1.0, 0.15, 0.15),
            ),
            EnemyType::Trooper => (
                90.0,
                0.8,
                2.2,
                6.5,
                250,
                Vec3::new(0.9, 1.8, 0.7),
                Vec3::new(0.2, 0.22, 0.28),
                Vec3::new(1.0, 0.2, 0.0),
            ),
            EnemyType::Titan => (
                260.0,
                1.6,
                3.4,
                4.5,
                1000,
                Vec3::new(2.2, 3.2, 1.8),
                Vec3::new(0.15, 0.15, 0.2),
                Vec3::new(0.9, 0.0, 1.0),
            ),
        };

        let default_mat = Rc::new(engine.assets().new_material("basic").with_color(main_color));
        let flash_mat = Rc::new(engine.assets().new_material("glow").with_color(Vec3::new(1.0, 1.0, 1.0)));
        let eye_mat = Rc::new(engine.assets().new_material("glow").with_color(eye_color));

        let mut tr_main = Transform::default();
        tr_main.position = spawn_pos;
        tr_main.scale = main_scale;
        let main_entity = engine.spawn(self.cube_mesh.clone(), default_mat.clone(), tr_main);

        // Eye / Visor entity
        let mut tr_eye = Transform::default();
        tr_eye.position = spawn_pos + Vec3::new(0.0, height * 0.4, 0.4);
        tr_eye.scale = match enemy_type {
            EnemyType::Drone => Vec3::new(0.3, 0.3, 0.2),
            EnemyType::Trooper => Vec3::new(0.5, 0.15, 0.2),
            EnemyType::Titan => Vec3::new(0.9, 0.3, 0.3),
        };
        let eye_entity = engine.spawn(self.cube_mesh.clone(), eye_mat.clone(), tr_eye);

        // Extra decorative part (e.g. cannon for trooper, core for titan)
        let extra_entity = match enemy_type {
            EnemyType::Trooper => {
                let cannon_mat = Rc::new(engine.assets().new_material("basic").with_color(Vec3::new(0.4, 0.4, 0.45)));
                let mut tr_cannon = Transform::default();
                tr_cannon.position = spawn_pos + Vec3::new(0.55, 0.0, 0.4);
                tr_cannon.scale = Vec3::new(0.2, 0.2, 0.9);
                Some(engine.spawn(self.cube_mesh.clone(), cannon_mat, tr_cannon))
            }
            EnemyType::Titan => {
                let mut tr_core = Transform::default();
                tr_core.position = spawn_pos + Vec3::new(0.0, 0.2, 0.9);
                tr_core.scale = Vec3::new(0.8, 0.8, 0.4);
                Some(engine.spawn(self.cube_mesh.clone(), eye_mat, tr_core))
            }
            _ => None,
        };

        self.enemies.push(Enemy {
            enemy_type,
            position: spawn_pos,
            velocity: Vec3::ZERO,
            yaw: rng.gen_range(-180.0..180.0),
            health,
            max_health: health,
            radius,
            height,
            speed,
            state: EnemyState::Pursuit,
            state_timer: rng.gen_range(2.0..4.0),
            attack_cooldown: rng.gen_range(0.5..1.5),
            burst_count: 0,
            burst_timer: 0.0,
            hit_flash_timer: 0.0,
            strafe_dir: if rng.gen_bool(0.5) { 1.0 } else { -1.0 },
            score_value,
            main_entity,
            eye_entity,
            extra_entity,
            default_mat,
            flash_mat,
        });
    }

    pub fn start_wave(&mut self, engine: &mut Engine, wave: u32) {
        self.current_wave = wave;
        self.wave_cleared = false;

        let spawn_radius = 26.0;
        let mut rng = rand::thread_rng();

        let (drone_count, trooper_count, titan_count) = match wave {
            1 => (4, 0, 0),
            2 => (3, 2, 0),
            3 => (4, 4, 0),
            4 => (3, 3, 1), // Mini-boss wave!
            w => (4 + w as usize, 3 + (w as usize / 2), (w as usize / 3)),
        };

        let total = drone_count + trooper_count + titan_count;
        let angle_step = std::f32::consts::TAU / (total.max(1) as f32);

        let mut idx = 0;
        for _ in 0..drone_count {
            let angle = (idx as f32) * angle_step + rng.gen_range(-0.2..0.2);
            let pos = Vec3::new(angle.cos() * spawn_radius, -5.2, angle.sin() * spawn_radius);
            self.spawn_enemy(engine, EnemyType::Drone, pos);
            idx += 1;
        }

        for _ in 0..trooper_count {
            let angle = (idx as f32) * angle_step + rng.gen_range(-0.2..0.2);
            let pos = Vec3::new(angle.cos() * spawn_radius, -7.0 + 1.1, angle.sin() * spawn_radius);
            self.spawn_enemy(engine, EnemyType::Trooper, pos);
            idx += 1;
        }

        for _ in 0..titan_count {
            let angle = (idx as f32) * angle_step + rng.gen_range(-0.2..0.2);
            let pos = Vec3::new(angle.cos() * spawn_radius, -7.0 + 1.7, angle.sin() * spawn_radius);
            self.spawn_enemy(engine, EnemyType::Titan, pos);
            idx += 1;
        }
    }

    pub fn apply_damage(
        &mut self,
        engine: &mut Engine,
        particles: &mut ParticleSystem,
        idx: usize,
        damage: f32,
    ) -> bool {
        if idx >= self.enemies.len() {
            return false;
        }

        let e = &mut self.enemies[idx];
        e.health -= damage;
        e.hit_flash_timer = 0.12;

        engine.object_mut(e.main_entity).set_material(e.flash_mat.clone());

        if e.health <= 0.0 {
            // Death explosion!
            self.score += e.score_value;
            self.total_kills += 1;

            let death_pos = e.position;
            particles.spawn_explosion(engine, death_pos, 4.5);
            particles.spawn_debris(engine, death_pos, Vec3::new(0.3, 0.35, 0.4));

            self.despawn_enemy_entities(engine, idx);
            self.enemies.remove(idx);
            true
        } else {
            false
        }
    }

    pub fn apply_splash_damage(
        &mut self,
        engine: &mut Engine,
        particles: &mut ParticleSystem,
        epicenter: Vec3,
        radius: f32,
        max_damage: f32,
    ) {
        let mut dead_indices = Vec::new();

        for (i, e) in self.enemies.iter_mut().enumerate() {
            let dist = (e.position - epicenter).length();
            if dist <= radius {
                let falloff = 1.0 - (dist / radius);
                let dmg = max_damage * falloff;
                e.health -= dmg;
                e.hit_flash_timer = 0.15;
                engine.object_mut(e.main_entity).set_material(e.flash_mat.clone());

                // Knockback
                let kb_dir = (e.position - epicenter).normalize_or_zero();
                e.velocity += kb_dir * (18.0 * falloff);

                if e.health <= 0.0 {
                    dead_indices.push(i);
                }
            }
        }

        for &idx in dead_indices.iter().rev() {
            let e = &self.enemies[idx];
            self.score += e.score_value;
            self.total_kills += 1;
            let death_pos = e.position;
            particles.spawn_explosion(engine, death_pos, 5.0);
            particles.spawn_debris(engine, death_pos, Vec3::new(0.35, 0.35, 0.4));
            self.despawn_enemy_entities(engine, idx);
            self.enemies.remove(idx);
        }
    }

    fn despawn_enemy_entities(&self, engine: &mut Engine, idx: usize) {
        let e = &self.enemies[idx];
        engine.despawn(e.main_entity);
        engine.despawn(e.eye_entity);
        if let Some(extra) = e.extra_entity {
            engine.despawn(extra);
        }
    }

    pub fn update(
        &mut self,
        engine: &mut Engine,
        arena: &Arena,
        projectiles: &mut ProjectileManager,
        player_pos: Vec3,
        player_vel: Vec3,
        dt: f32,
    ) {
        // Wave management
        if self.enemies.is_empty() {
            if !self.wave_cleared {
                self.wave_cleared = true;
                self.wave_intermission_timer = 3.0;
            } else {
                self.wave_intermission_timer -= dt;
                if self.wave_intermission_timer <= 0.0 {
                    self.start_wave(engine, self.current_wave + 1);
                }
            }
        }

        let mut rng = rand::thread_rng();

        for i in 0..self.enemies.len() {
            let e = &mut self.enemies[i];

            // Hit flash timer
            if e.hit_flash_timer > 0.0 {
                e.hit_flash_timer -= dt;
                if e.hit_flash_timer <= 0.0 {
                    engine.object_mut(e.main_entity).set_material(e.default_mat.clone());
                }
            }

            // Vector to player
            let to_player = player_pos - e.position;
            let dist_to_player = to_player.length();
            let dir_to_player = if dist_to_player > 0.001 { to_player / dist_to_player } else { Vec3::Z };

            // Rotate toward player smoothly
            let target_yaw = dir_to_player.z.atan2(dir_to_player.x).to_degrees();
            let mut diff = target_yaw - e.yaw;
            while diff < -180.0 { diff += 360.0; }
            while diff > 180.0 { diff -= 360.0; }
            e.yaw += diff * (8.0 * dt).min(1.0);

            // State Machine
            e.state_timer -= dt;
            if e.state_timer <= 0.0 {
                e.state_timer = rng.gen_range(2.0..4.5);
                // Invert strafe direction occasionally
                e.strafe_dir = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
            }

            // Movement behavior per type
            let planar_forward = Vec3::new(dir_to_player.x, 0.0, dir_to_player.z).normalize_or_zero();
            let planar_right = planar_forward.cross(Vec3::Y).normalize_or_zero();

            let move_vec;

            match e.enemy_type {
                EnemyType::Drone => {
                    // Aggressive, closes in while weaving left/right
                    if dist_to_player > 6.0 {
                        move_vec = planar_forward * e.speed + planar_right * (e.strafe_dir * e.speed * 0.7);
                    } else {
                        // Circle strafe when close
                        move_vec = planar_right * (e.strafe_dir * e.speed);
                    }

                    // Hover bobbing
                    let target_y = -7.0 + 1.8 + (e.position.x * 2.0).sin() * 0.4;
                    e.position.y += (target_y - e.position.y) * (4.0 * dt).min(1.0);

                    // Attack logic: rapid red dart
                    e.attack_cooldown -= dt;
                    if e.attack_cooldown <= 0.0 && dist_to_player < 22.0 {
                        e.attack_cooldown = rng.gen_range(0.9..1.6);
                        let shoot_dir = (dir_to_player + Vec3::new(
                            rng.gen_range(-0.06..0.06),
                            rng.gen_range(-0.04..0.04),
                            rng.gen_range(-0.06..0.06),
                        )).normalize();

                        projectiles.spawn(
                            engine,
                            ProjectileOwner::Enemy,
                            e.position + dir_to_player * 0.6,
                            shoot_dir,
                            45.0,
                            12.0,
                            false,
                            0.0,
                            0.0,
                            Vec3::new(1.0, 0.15, 0.15),
                        );
                    }
                }
                EnemyType::Trooper => {
                    // Tactical standoff: keep 12-18m distance, strafe and burst fire
                    if dist_to_player > 16.0 {
                        move_vec = planar_forward * e.speed;
                    } else if dist_to_player < 8.0 {
                        move_vec = -planar_forward * (e.speed * 0.8) + planar_right * (e.strafe_dir * e.speed * 0.6);
                    } else {
                        move_vec = planar_right * (e.strafe_dir * e.speed);
                    }

                    // Gravity
                    e.velocity.y -= 25.0 * dt;
                    e.position.y += e.velocity.y * dt;
                    if e.position.y <= -7.0 + e.height * 0.5 {
                        e.position.y = -7.0 + e.height * 0.5;
                        e.velocity.y = 0.0;
                    }

                    // Burst fire rifle logic
                    e.attack_cooldown -= dt;
                    if e.burst_count > 0 {
                        e.burst_timer -= dt;
                        if e.burst_timer <= 0.0 {
                            e.burst_timer = 0.14;
                            e.burst_count -= 1;

                            // Predictive lead shot!
                            let lead_pos = player_pos + player_vel * (dist_to_player / 55.0);
                            let shoot_dir = (lead_pos - e.position).normalize();

                            projectiles.spawn(
                                engine,
                                ProjectileOwner::Enemy,
                                e.position + dir_to_player * 0.8 + Vec3::new(0.0, 0.3, 0.0),
                                shoot_dir,
                                55.0,
                                14.0,
                                false,
                                0.0,
                                0.0,
                                Vec3::new(1.0, 0.4, 0.0),
                            );
                        }
                    } else if e.attack_cooldown <= 0.0 && dist_to_player < 28.0 {
                        e.attack_cooldown = rng.gen_range(1.6..2.6);
                        e.burst_count = 3;
                        e.burst_timer = 0.0;
                    }
                }
                EnemyType::Titan => {
                    // Slow unstoppable tank, fires tracking rockets!
                    let enraged = e.health < (e.max_health * 0.5);
                    let current_speed = if enraged { e.speed * 1.5 } else { e.speed };

                    move_vec = planar_forward * current_speed;

                    // Gravity
                    e.velocity.y -= 25.0 * dt;
                    e.position.y += e.velocity.y * dt;
                    if e.position.y <= -7.0 + e.height * 0.5 {
                        e.position.y = -7.0 + e.height * 0.5;
                        e.velocity.y = 0.0;
                    }

                    e.attack_cooldown -= dt;
                    let fire_interval = if enraged { 1.2 } else { 2.2 };

                    if e.attack_cooldown <= 0.0 && dist_to_player < 35.0 {
                        e.attack_cooldown = fire_interval;

                        // Fire rocket from shoulder pod
                        let shoulder_offset = planar_right * 1.1 + Vec3::new(0.0, 1.2, 0.0);
                        let spawn_pt = e.position + shoulder_offset;
                        let shoot_dir = (player_pos - spawn_pt).normalize();

                        projectiles.spawn(
                            engine,
                            ProjectileOwner::Enemy,
                            spawn_pt,
                            shoot_dir,
                            32.0,
                            35.0,
                            true,
                            5.0,
                            50.0,
                            Vec3::new(0.9, 0.1, 0.9),
                        );
                    }
                }
            }

            // Apply friction & move
            e.velocity.x = move_vec.x;
            e.velocity.z = move_vec.z;
            e.position += Vec3::new(e.velocity.x, 0.0, e.velocity.z) * dt;

            // Arena wall collision
            arena.resolve_player_collision(&mut e.position, e.radius, e.height);

            // Update visual transforms
            let yaw_rad = (-e.yaw - 90.0).to_radians();
            let rot = Vec3::new(0.0, yaw_rad, 0.0);

            let tr_main = engine.transform_mut(e.main_entity);
            tr_main.position = e.position;
            tr_main.rotation = rot;

            // Eye transform attached to forward facing
            let eye_offset = planar_forward * (e.radius * 0.9) + Vec3::new(0.0, e.height * 0.3, 0.0);
            let tr_eye = engine.transform_mut(e.eye_entity);
            tr_eye.position = e.position + eye_offset;
            tr_eye.rotation = rot;

            if let Some(extra) = e.extra_entity {
                let tr_extra = engine.transform_mut(extra);
                tr_extra.position = e.position + planar_forward * (e.radius * 0.7) + planar_right * 0.6;
                tr_extra.rotation = rot;
            }
        }
    }

    pub fn enemy_targets(&self) -> Vec<(usize, Vec3, f32)> {
        self.enemies
            .iter()
            .enumerate()
            .map(|(i, e)| (i, e.position, e.radius))
            .collect()
    }
}
