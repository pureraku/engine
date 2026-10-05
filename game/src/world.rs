use crate::ai::AiManager;
use crate::arena::Arena;
use crate::hud::Hud;
use crate::particles::ParticleSystem;
use crate::pickups::PickupManager;
use crate::player::Player;
use crate::projectiles::{ProjectileManager, ProjectileOwner};
use crate::weapons::Weapons;
use engine::{Engine, Game, Key, Vec3};
use rand::Rng;

pub struct World {
    arena: Option<Arena>,
    player: Option<Player>,
    weapons: Option<Weapons>,
    projectiles: ProjectileManager,
    particles: Option<ParticleSystem>,
    pickups: Option<PickupManager>,
    ai: Option<AiManager>,
    hud: Hud,
}

impl Default for World {
    fn default() -> Self {
        Self {
            arena: None,
            player: None,
            weapons: None,
            projectiles: ProjectileManager::new(),
            particles: None,
            pickups: None,
            ai: None,
            hud: Hud::new(),
        }
    }
}

impl Game for World {
    fn init(&mut self, engine: &mut Engine) {
        // Use custom camera controller so Player controls the camera
        engine.set_custom_camera(true);

        // Cyber arena lighting
        engine.lighting().light_pos = Vec3::new(0.0, 18.0, 0.0);
        engine.lighting().light_color = Vec3::new(0.9, 0.95, 1.0);
        engine.lighting().light_intensity = 2.8;

        // Instantiate systems
        let arena = Arena::new(engine);
        let player = Player::new(Vec3::new(0.0, -7.0, 18.0));
        let weapons = Weapons::new(engine);
        let particles = ParticleSystem::new(engine, 96);
        let pickups = PickupManager::new(engine);

        let mut ai = AiManager::new(engine);
        ai.start_wave(engine, 1);

        self.arena = Some(arena);
        self.player = Some(player);
        self.weapons = Some(weapons);
        self.particles = Some(particles);
        self.pickups = Some(pickups);
        self.ai = Some(ai);
    }

    fn update(&mut self, engine: &mut Engine, time: f32, dt: f32) {
        let (arena, player, weapons, particles, pickups, ai) = match (
            &self.arena,
            &mut self.player,
            &mut self.weapons,
            &mut self.particles,
            &mut self.pickups,
            &mut self.ai,
        ) {
            (
                Some(ar),
                Some(pl),
                Some(wp),
                Some(pt),
                Some(pk),
                Some(ai),
            ) => (ar, pl, wp, pt, pk, ai),
            _ => return,
        };

        // Handle Restart on 'R' if player died
        if !player.is_alive() && engine.input().is_key_pressed(Key::R) {
            *player = Player::new(Vec3::new(0.0, -7.0, 18.0));
            *weapons = Weapons::new(engine);
            self.projectiles.clear(engine);

            // Clean up old enemy entities before starting wave 1
            for e in ai.enemies.drain(..) {
                engine.despawn(e.main_entity);
                engine.despawn(e.eye_entity);
                if let Some(extra) = e.extra_entity {
                    engine.despawn(extra);
                }
            }
            ai.total_kills = 0;
            ai.score = 0;
            ai.start_wave(engine, 1);
            return;
        }

        // 1. Update Player Physics and Movement
        player.update(engine.input(), arena, dt);
        player.apply_to_camera(engine.camera_mut());

        // 2. Update Weapons and Handle Firing
        let camera_copy = engine.camera().clone();
        let fired_shots = weapons.update(engine, &camera_copy, dt);

        let mut rng = rand::thread_rng();
        for shot in fired_shots {
            self.projectiles.spawn(
                engine,
                ProjectileOwner::Player,
                shot.origin,
                shot.direction,
                shot.speed,
                shot.damage,
                shot.is_rocket,
                shot.splash_radius,
                shot.splash_damage,
                shot.color,
            );

            // Recoil impulse on player
            let active_w = weapons.active();
            let yaw_kick = rng.gen_range(-active_w.recoil_yaw..active_w.recoil_yaw);
            player.add_recoil(active_w.recoil_pitch, yaw_kick);
        }

        // 3. Update AI Opponents
        ai.update(
            engine,
            arena,
            &mut self.projectiles,
            player.position,
            player.velocity,
            dt,
        );

        // 4. Update Projectiles & Check Collisions
        let enemy_targets = ai.enemy_targets();
        let hit_events = self.projectiles.update(
            engine,
            arena,
            particles,
            player.position,
            player.radius,
            &enemy_targets,
            dt,
        );

        for hit in hit_events {
            if let Some(enemy_idx) = hit.target_enemy_idx {
                ai.apply_damage(engine, particles, enemy_idx, hit.damage);
                self.hud.trigger_hitmarker();
            }

            if hit.hit_player {
                player.take_damage(hit.damage);
            }

            if hit.is_rocket {
                // Splash damage on enemies
                ai.apply_splash_damage(
                    engine,
                    particles,
                    hit.hit_pos,
                    hit.splash_radius,
                    hit.splash_damage,
                );

                // Splash damage on player
                let d_player = (player.position - hit.hit_pos).length();
                if d_player <= hit.splash_radius {
                    let falloff = 1.0 - (d_player / hit.splash_radius);
                    player.take_damage(hit.splash_damage * falloff);

                    // Blast knockback
                    let blast_dir = (player.position - hit.hit_pos).normalize_or_zero();
                    player.velocity += blast_dir * (20.0 * falloff) + Vec3::new(0.0, 6.0 * falloff, 0.0);
                }
            }
        }

        // 5. Update Particles
        particles.update(engine, dt);

        // 6. Update Pickups
        pickups.update(engine, player, weapons, time, dt);

        // 7. Update and Draw HUD
        self.hud.update(dt);
        self.hud.draw(
            engine.ui(),
            player,
            weapons,
            ai,
            pickups,
            arena,
            time,
        );
    }
}
