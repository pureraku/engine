use crate::player::Player;
use crate::weapons::Weapons;
use engine::assets::asset_manager::MeshType;
use engine::{Engine, EntityId, Vec3, transform::Transform};
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PickupType {
    Health,
    Armor,
    Ammo,
}

pub struct Pickup {
    pub entity: EntityId,
    pub pickup_type: PickupType,
    pub base_position: Vec3,
    pub active: bool,
    pub respawn_timer: f32,
    pub respawn_delay: f32,
    pub rotation: f32,
}

pub struct PickupManager {
    pickups: Vec<Pickup>,
}

impl PickupManager {
    pub fn new(engine: &mut Engine) -> Self {
        let cube_mesh = engine.assets().mesh(MeshType::Cube);
        let ground_y = -7.0;

        let spawn_configs = [
            (PickupType::Health, Vec3::new(-16.0, ground_y + 1.2, -16.0), Vec3::new(0.1, 1.0, 0.3)),
            (PickupType::Armor, Vec3::new(16.0, ground_y + 1.2, 16.0), Vec3::new(0.1, 0.8, 1.0)),
            (PickupType::Ammo, Vec3::new(-16.0, ground_y + 1.2, 16.0), Vec3::new(1.0, 0.9, 0.1)),
            (PickupType::Health, Vec3::new(16.0, ground_y + 1.2, -16.0), Vec3::new(0.1, 1.0, 0.3)),
            (PickupType::Ammo, Vec3::new(0.0, ground_y + 1.2, -26.0), Vec3::new(1.0, 0.9, 0.1)),
            (PickupType::Armor, Vec3::new(0.0, ground_y + 1.2, 26.0), Vec3::new(0.1, 0.8, 1.0)),
        ];

        let mut pickups = Vec::new();
        for (p_type, pos, color) in spawn_configs {
            let mat = Rc::new(engine.assets().new_material("glow").with_color(color));
            let mut tr = Transform::default();
            tr.position = pos;
            tr.scale = match p_type {
                PickupType::Health => Vec3::new(0.7, 0.7, 0.7),
                PickupType::Armor => Vec3::new(0.65, 0.85, 0.65),
                PickupType::Ammo => Vec3::new(0.8, 0.5, 0.8),
            };

            let entity = engine.spawn(cube_mesh.clone(), mat, tr);

            pickups.push(Pickup {
                entity,
                pickup_type: p_type,
                base_position: pos,
                active: true,
                respawn_timer: 0.0,
                respawn_delay: 14.0,
                rotation: 0.0,
            });
        }

        Self { pickups }
    }

    pub fn update(
        &mut self,
        engine: &mut Engine,
        player: &mut Player,
        weapons: &mut Weapons,
        time: f32,
        dt: f32,
    ) {
        for p in &mut self.pickups {
            if !p.active {
                p.respawn_timer -= dt;
                if p.respawn_timer <= 0.0 {
                    p.active = true;
                    engine.set_visible(p.entity, true);
                }
                continue;
            }

            // Animate floating and spinning
            p.rotation += 2.0 * dt;
            let bob = (time * 3.0 + p.base_position.x).sin() * 0.25;

            let tr = engine.transform_mut(p.entity);
            tr.position = p.base_position + Vec3::new(0.0, bob, 0.0);
            tr.rotation.y = p.rotation;
            tr.rotation.x = (time * 1.5).cos() * 0.15;

            // Player pickup detection
            let dist_sq = (player.position - tr.position).length_squared();
            if dist_sq < 2.2 * 2.2 {
                let collected = match p.pickup_type {
                    PickupType::Health => {
                        if player.health < player.max_health {
                            player.heal(45.0);
                            true
                        } else {
                            false
                        }
                    }
                    PickupType::Armor => {
                        if player.armor < player.max_armor {
                            player.add_armor(45.0);
                            true
                        } else {
                            false
                        }
                    }
                    PickupType::Ammo => {
                        weapons.add_all_ammo(50, 12, 3);
                        true
                    }
                };

                if collected {
                    p.active = false;
                    p.respawn_timer = p.respawn_delay;
                    engine.set_visible(p.entity, false);
                }
            }
        }
    }

    pub fn active_pickups(&self) -> impl Iterator<Item = (&Pickup, Vec3)> {
        self.pickups.iter().filter(|p| p.active).map(|p| (p, p.base_position))
    }
}
