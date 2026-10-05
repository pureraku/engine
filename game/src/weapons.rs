use engine::assets::asset_manager::MeshType;
use engine::{Camera, Engine, EntityId, Key, MouseButton, Vec3, transform::Transform};
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeaponType {
    PulseBlaster,
    ScatterShotgun,
    PlasmaRocket,
}

#[derive(Clone, Debug)]
pub struct WeaponData {
    pub name: &'static str,
    pub weapon_type: WeaponType,
    pub ammo_in_mag: i32,
    pub mag_capacity: i32,
    pub ammo_reserve: i32,
    pub max_reserve: i32,
    pub fire_delay: f32,
    pub reload_time: f32,
    pub damage: f32,
    pub speed: f32,
    pub recoil_pitch: f32,
    pub recoil_yaw: f32,
    pub color: Vec3,
}

pub struct FiredShot {
    pub origin: Vec3,
    pub direction: Vec3,
    pub damage: f32,
    pub speed: f32,
    pub is_rocket: bool,
    pub splash_radius: f32,
    pub splash_damage: f32,
    pub color: Vec3,
}

#[allow(dead_code)]
pub struct Weapons {
    pub weapons: [WeaponData; 3],
    pub active_index: usize,

    pub fire_timer: f32,
    pub reload_timer: f32,
    pub is_reloading: bool,

    // Recoil kick for viewmodel
    pub viewmodel_kick_z: f32,
    pub viewmodel_kick_pitch: f32,
    pub muzzle_flash_timer: f32,

    // 3D Viewmodel entities
    body_entity: EntityId,
    barrel_entity: EntityId,
    core_entity: EntityId,
    muzzle_entity: EntityId,

    core_material: Rc<engine::assets::material::Material>,
    muzzle_material: Rc<engine::assets::material::Material>,
}

impl Weapons {
    pub fn new(engine: &mut Engine) -> Self {
        let blaster = WeaponData {
            name: "PULSE BLASTER",
            weapon_type: WeaponType::PulseBlaster,
            ammo_in_mag: 30,
            mag_capacity: 30,
            ammo_reserve: 150,
            max_reserve: 240,
            fire_delay: 0.12,
            reload_time: 1.2,
            damage: 22.0,
            speed: 80.0,
            recoil_pitch: 0.9,
            recoil_yaw: 0.25,
            color: Vec3::new(0.0, 0.9, 1.0),
        };

        let shotgun = WeaponData {
            name: "SCATTER SHOTGUN",
            weapon_type: WeaponType::ScatterShotgun,
            ammo_in_mag: 8,
            mag_capacity: 8,
            ammo_reserve: 32,
            max_reserve: 48,
            fire_delay: 0.65,
            reload_time: 1.6,
            damage: 15.0, // per pellet (7 pellets)
            speed: 75.0,
            recoil_pitch: 3.2,
            recoil_yaw: 0.5,
            color: Vec3::new(1.0, 0.6, 0.1),
        };

        let rocket = WeaponData {
            name: "PLASMA ROCKET",
            weapon_type: WeaponType::PlasmaRocket,
            ammo_in_mag: 4,
            mag_capacity: 4,
            ammo_reserve: 12,
            max_reserve: 20,
            fire_delay: 1.0,
            reload_time: 2.0,
            damage: 100.0,
            speed: 42.0,
            recoil_pitch: 4.5,
            recoil_yaw: 0.1,
            color: Vec3::new(1.0, 0.2, 0.8),
        };

        let cube_mesh = engine.assets().mesh(MeshType::Cube);
        let gun_mat = Rc::new(
            engine.assets().new_material("basic").with_color(Vec3::new(0.12, 0.14, 0.18)),
        );
        let barrel_mat = Rc::new(
            engine.assets().new_material("basic").with_color(Vec3::new(0.25, 0.28, 0.32)),
        );
        let core_material = Rc::new(
            engine.assets().new_material("glow").with_color(blaster.color),
        );
        let muzzle_material = Rc::new(
            engine.assets().new_material("glow").with_color(Vec3::new(1.0, 1.0, 0.6)),
        );

        let body_entity = engine.spawn(cube_mesh.clone(), gun_mat, Transform::default());
        let barrel_entity = engine.spawn(cube_mesh.clone(), barrel_mat, Transform::default());
        let core_entity = engine.spawn(cube_mesh.clone(), core_material.clone(), Transform::default());
        let muzzle_entity = engine.spawn(cube_mesh, muzzle_material.clone(), Transform::default());

        Self {
            weapons: [blaster, shotgun, rocket],
            active_index: 0,
            fire_timer: 0.0,
            reload_timer: 0.0,
            is_reloading: false,
            viewmodel_kick_z: 0.0,
            viewmodel_kick_pitch: 0.0,
            muzzle_flash_timer: 0.0,
            body_entity,
            barrel_entity,
            core_entity,
            muzzle_entity,
            core_material,
            muzzle_material,
        }
    }

    pub fn active(&self) -> &WeaponData {
        &self.weapons[self.active_index]
    }

    pub fn active_mut(&mut self) -> &mut WeaponData {
        &mut self.weapons[self.active_index]
    }

    pub fn switch_to(&mut self, engine: &mut Engine, index: usize) {
        if index != self.active_index && index < self.weapons.len() {
            self.active_index = index;
            self.is_reloading = false;
            self.reload_timer = 0.0;
            self.fire_timer = 0.15; // Brief switch delay

            // Update core glowing color to match weapon
            let col = self.weapons[index].color;
            let new_core_mat = Rc::new(engine.assets().new_material("glow").with_color(col));
            engine.object_mut(self.core_entity).set_material(new_core_mat.clone());
            self.core_material = new_core_mat;
        }
    }

    pub fn start_reload(&mut self) {
        let (ammo_in_mag, mag_capacity, ammo_reserve, reload_time) = {
            let w = self.active();
            (w.ammo_in_mag, w.mag_capacity, w.ammo_reserve, w.reload_time)
        };
        if !self.is_reloading && ammo_in_mag < mag_capacity && ammo_reserve > 0 {
            self.is_reloading = true;
            self.reload_timer = reload_time;
        }
    }

    pub fn add_ammo(&mut self, weapon_type: WeaponType, amount: i32) {
        for w in &mut self.weapons {
            if w.weapon_type == weapon_type {
                w.ammo_reserve = (w.ammo_reserve + amount).min(w.max_reserve);
            }
        }
    }

    pub fn add_all_ammo(&mut self, blaster: i32, shotgun: i32, rocket: i32) {
        self.add_ammo(WeaponType::PulseBlaster, blaster);
        self.add_ammo(WeaponType::ScatterShotgun, shotgun);
        self.add_ammo(WeaponType::PlasmaRocket, rocket);
    }

    pub fn update(
        &mut self,
        engine: &mut Engine,
        camera: &Camera,
        dt: f32,
    ) -> Vec<FiredShot> {
        let mut fired_shots = Vec::new();

        self.fire_timer = (self.fire_timer - dt).max(0.0);
        self.muzzle_flash_timer = (self.muzzle_flash_timer - dt).max(0.0);

        // Extract input cleanly without persistent engine borrow
        let (num1, num2, num3, scroll_y, reload_key, shoot_btn) = {
            let inp = engine.input();
            (
                inp.is_key_pressed(Key::Num1),
                inp.is_key_pressed(Key::Num2),
                inp.is_key_pressed(Key::Num3),
                inp.scroll_delta.y,
                inp.is_key_pressed(Key::R),
                inp.is_mouse_down(MouseButton::Button1),
            )
        };

        // Weapon switching via 1, 2, 3 or Scroll
        if num1 {
            self.switch_to(engine, 0);
        } else if num2 {
            self.switch_to(engine, 1);
        } else if num3 {
            self.switch_to(engine, 2);
        }

        if scroll_y > 0.0 {
            let next = (self.active_index + 1) % self.weapons.len();
            self.switch_to(engine, next);
        } else if scroll_y < 0.0 {
            let prev = if self.active_index == 0 { self.weapons.len() - 1 } else { self.active_index - 1 };
            self.switch_to(engine, prev);
        }

        // Reload input
        if reload_key {
            self.start_reload();
        }

        // Handle reload countdown
        if self.is_reloading {
            self.reload_timer -= dt;
            if self.reload_timer <= 0.0 {
                self.is_reloading = false;
                let w = self.active_mut();
                let needed = w.mag_capacity - w.ammo_in_mag;
                let take = needed.min(w.ammo_reserve);
                w.ammo_in_mag += take;
                w.ammo_reserve -= take;
            }
        }

        // Firing logic
        if shoot_btn && !self.is_reloading && self.fire_timer <= 0.0 {
            let w = self.active();
            if w.ammo_in_mag > 0 {
                let forward = camera.front();
                let right = camera.right();
                let up = camera.up();

                // Origin slightly below and right of eye
                let muzzle_origin = camera.position + forward * 0.6 + right * 0.25 - up * 0.18;

                match w.weapon_type {
                    WeaponType::PulseBlaster => {
                        fired_shots.push(FiredShot {
                            origin: muzzle_origin,
                            direction: forward,
                            damage: w.damage,
                            speed: w.speed,
                            is_rocket: false,
                            splash_radius: 0.0,
                            splash_damage: 0.0,
                            color: w.color,
                        });
                    }
                    WeaponType::ScatterShotgun => {
                        let spreads = [
                            (0.0, 0.0),
                            (0.035, 0.02),
                            (-0.035, 0.02),
                            (0.04, -0.025),
                            (-0.04, -0.025),
                            (0.015, -0.04),
                            (-0.015, 0.04),
                        ];
                        for (ox, oy) in spreads {
                            let pellet_dir = (forward + right * ox + up * oy).normalize();
                            fired_shots.push(FiredShot {
                                origin: muzzle_origin,
                                direction: pellet_dir,
                                damage: w.damage,
                                speed: w.speed,
                                is_rocket: false,
                                splash_radius: 0.0,
                                splash_damage: 0.0,
                                color: w.color,
                            });
                        }
                    }
                    WeaponType::PlasmaRocket => {
                        fired_shots.push(FiredShot {
                            origin: muzzle_origin,
                            direction: forward,
                            damage: w.damage,
                            speed: w.speed,
                            is_rocket: true,
                            splash_radius: 6.0,
                            splash_damage: 120.0,
                            color: w.color,
                        });
                    }
                }

                // Recoil kick
                self.viewmodel_kick_z = 0.15;
                self.viewmodel_kick_pitch = 0.18;
                self.muzzle_flash_timer = 0.06;

                let delay = self.active().fire_delay;
                self.fire_timer = delay;
                self.active_mut().ammo_in_mag -= 1;

                if self.active().ammo_in_mag == 0 && self.active().ammo_reserve > 0 {
                    self.start_reload();
                }
            } else if w.ammo_reserve > 0 {
                self.start_reload();
            }
        }

        // Viewmodel recoil recovery
        let spring = (14.0 * dt).min(1.0);
        self.viewmodel_kick_z -= self.viewmodel_kick_z * spring;
        self.viewmodel_kick_pitch -= self.viewmodel_kick_pitch * spring;

        // Position 3D Viewmodel relative to Camera
        self.update_viewmodel(engine, camera);

        fired_shots
    }

    fn update_viewmodel(&self, engine: &mut Engine, camera: &Camera) {
        let forward = camera.front();
        let right = camera.right();
        let up = camera.up();

        // Gun base offset in camera view
        let base_offset = right * 0.28 - up * 0.24 + forward * (0.55 - self.viewmodel_kick_z);
        let gun_center = camera.position + base_offset;

        let yaw_rad = (-camera.yaw - 90.0).to_radians();
        let pitch_rad = (-camera.pitch).to_radians() + self.viewmodel_kick_pitch;

        let rot = Vec3::new(pitch_rad, yaw_rad, 0.0);

        // 1. Receiver / Body
        let body_tr = engine.transform_mut(self.body_entity);
        body_tr.position = gun_center;
        body_tr.rotation = rot;
        body_tr.scale = Vec3::new(0.08, 0.12, 0.32);

        // 2. Barrel
        let barrel_pos = gun_center + forward * 0.22 - up * 0.02;
        let barrel_tr = engine.transform_mut(self.barrel_entity);
        barrel_tr.position = barrel_pos;
        barrel_tr.rotation = rot;
        barrel_tr.scale = match self.active().weapon_type {
            WeaponType::PulseBlaster => Vec3::new(0.05, 0.05, 0.28),
            WeaponType::ScatterShotgun => Vec3::new(0.09, 0.06, 0.22),
            WeaponType::PlasmaRocket => Vec3::new(0.12, 0.12, 0.24),
        };

        // 3. Glowing Core / Battery
        let core_pos = gun_center - up * 0.04 - forward * 0.05;
        let core_tr = engine.transform_mut(self.core_entity);
        core_tr.position = core_pos;
        core_tr.rotation = rot;
        core_tr.scale = Vec3::new(0.06, 0.08, 0.14);

        // 4. Muzzle Flash
        let muzzle_tr = engine.transform_mut(self.muzzle_entity);
        if self.muzzle_flash_timer > 0.0 {
            muzzle_tr.position = barrel_pos + forward * 0.18;
            muzzle_tr.rotation = rot;
            muzzle_tr.scale = Vec3::splat(0.15);
            engine.set_visible(self.muzzle_entity, true);
        } else {
            engine.set_visible(self.muzzle_entity, false);
        }
    }
}
