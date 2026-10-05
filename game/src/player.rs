use crate::arena::Arena;
use engine::{Camera, Input, Key, Vec3};

pub struct Player {
    pub position: Vec3,
    pub velocity: Vec3,
    pub yaw: f32,
    pub pitch: f32,

    pub eye_height: f32,
    pub radius: f32,

    pub health: f32,
    pub max_health: f32,
    pub armor: f32,
    pub max_armor: f32,

    pub is_grounded: bool,
    pub move_speed: f32,
    pub sprint_speed: f32,
    pub jump_velocity: f32,
    pub gravity: f32,
    pub mouse_sensitivity: f32,

    // Recoil and camera effects
    pub recoil_pitch: f32,
    pub recoil_yaw: f32,
    pub bob_phase: f32,
    pub bob_offset: Vec3,
    pub damage_flash: f32,
    pub damage_shake: f32,
}

impl Player {
    pub fn new(spawn_pos: Vec3) -> Self {
        Self {
            position: spawn_pos,
            velocity: Vec3::ZERO,
            yaw: -90.0,
            pitch: 0.0,

            eye_height: 1.7,
            radius: 0.6,

            health: 500.0,
            max_health: 1000.0,
            armor: 500.0,
            max_armor: 500.0,

            is_grounded: false,
            move_speed: 9.0,
            sprint_speed: 14.5,
            jump_velocity: 9.5,
            gravity: 28.0,
            mouse_sensitivity: 0.12,

            recoil_pitch: 0.0,
            recoil_yaw: 0.0,
            bob_phase: 0.0,
            bob_offset: Vec3::ZERO,
            damage_flash: 0.0,
            damage_shake: 0.0,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0.0
    }

    pub fn eye_pos(&self) -> Vec3 {
        let mut shake = Vec3::ZERO;
        if self.damage_shake > 0.0 {
            let s = self.damage_shake;
            shake = Vec3::new(
                (s * 37.0).sin() * 0.1 * s,
                (s * 43.0).cos() * 0.1 * s,
                (s * 29.0).sin() * 0.05 * s,
            );
        }
        self.position + Vec3::new(0.0, self.eye_height, 0.0) + self.bob_offset + shake
    }

    #[allow(dead_code)]
    pub fn forward(&self) -> Vec3 {
        let total_yaw = (self.yaw + self.recoil_yaw).to_radians();
        let total_pitch = (self.pitch + self.recoil_pitch).clamp(-89.0, 89.0).to_radians();
        Vec3::new(
            total_yaw.cos() * total_pitch.cos(),
            total_pitch.sin(),
            total_yaw.sin() * total_pitch.cos(),
        )
        .normalize()
    }

    #[allow(dead_code)]
    pub fn right(&self) -> Vec3 {
        self.forward().cross(Vec3::Y).normalize()
    }

    pub fn take_damage(&mut self, amount: f32) -> bool {
        if self.health <= 0.0 {
            return true;
        }

        // Armor absorbs 60% of damage
        if self.armor > 0.0 {
            let absorb = (amount * 0.6).min(self.armor);
            self.armor -= absorb;
            let remaining = amount - absorb;
            self.health -= remaining;
        } else {
            self.health -= amount;
        }

        self.damage_flash = 0.35;
        self.damage_shake = 0.3;

        if self.health <= 0.0 {
            self.health = 0.0;
            true
        } else {
            false
        }
    }

    pub fn heal(&mut self, amount: f32) {
        self.health = (self.health + amount).min(self.max_health);
    }

    pub fn add_armor(&mut self, amount: f32) {
        self.armor = (self.armor + amount).min(self.max_armor);
    }

    pub fn add_recoil(&mut self, pitch: f32, yaw: f32) {
        self.recoil_pitch += pitch;
        self.recoil_yaw += yaw;
    }

    pub fn update(&mut self, input: &Input, arena: &Arena, dt: f32) {
        // Mouse Look
        let md = input.mouse_delta;
        self.yaw += md.x * self.mouse_sensitivity;
        self.pitch -= md.y * self.mouse_sensitivity;
        self.pitch = self.pitch.clamp(-89.0, 89.0);

        // Recoil decay (spring back smoothly)
        let decay = (15.0 * dt).min(1.0);
        self.recoil_pitch -= self.recoil_pitch * decay;
        self.recoil_yaw -= self.recoil_yaw * decay;

        // Damage flash and shake timers
        self.damage_flash = (self.damage_flash - dt).max(0.0);
        self.damage_shake = (self.damage_shake - dt).max(0.0);

        if self.health <= 0.0 {
            return;
        }

        // Horizontal Movement
        let yaw_rad = self.yaw.to_radians();
        let forward_planar = Vec3::new(yaw_rad.cos(), 0.0, yaw_rad.sin()).normalize();
        let right_planar = forward_planar.cross(Vec3::Y).normalize();

        let mut move_dir = Vec3::ZERO;
        if input.is_key_down(Key::W) {
            move_dir += forward_planar;
        }
        if input.is_key_down(Key::S) {
            move_dir -= forward_planar;
        }
        if input.is_key_down(Key::A) {
            move_dir -= right_planar;
        }
        if input.is_key_down(Key::D) {
            move_dir += right_planar;
        }

        let is_sprinting = input.is_key_down(Key::LeftShift) || input.is_key_down(Key::RightShift);
        let speed = if is_sprinting { self.sprint_speed } else { self.move_speed };

        let moving = move_dir.length_squared() > 0.0;
        if moving {
            move_dir = move_dir.normalize();
            self.velocity.x = move_dir.x * speed;
            self.velocity.z = move_dir.z * speed;

            // Weapon bobbing
            let bob_freq = if is_sprinting { 14.0 } else { 10.0 };
            self.bob_phase += dt * bob_freq;
            self.bob_offset = Vec3::new(
                (self.bob_phase * 0.5).sin() * 0.03,
                (self.bob_phase).sin().abs() * -0.04,
                0.0,
            );
        } else {
            // Friction damping
            let friction = (12.0 * dt).min(1.0);
            self.velocity.x -= self.velocity.x * friction;
            self.velocity.z -= self.velocity.z * friction;
            self.bob_offset = self.bob_offset * (1.0 - (10.0 * dt).min(1.0));
        }

        // Jump
        if self.is_grounded && input.is_key_pressed(Key::Space) {
            self.velocity.y = self.jump_velocity;
            self.is_grounded = false;
        }

        // Gravity
        self.velocity.y -= self.gravity * dt;

        // Apply movement
        self.position += self.velocity * dt;

        // Ground check
        if self.position.y <= arena.ground_y {
            self.position.y = arena.ground_y;
            self.velocity.y = 0.0;
            self.is_grounded = true;
        } else {
            self.is_grounded = false;
        }

        // Jump pad check
        if let Some(pad_power) = arena.check_jump_pads(self.position) {
            self.velocity.y = pad_power;
            self.is_grounded = false;
        }

        // Collision with walls and pillars
        arena.resolve_player_collision(&mut self.position, self.radius, self.eye_height);
    }

    pub fn apply_to_camera(&self, camera: &mut Camera) {
        camera.position = self.eye_pos();
        camera.yaw = self.yaw + self.recoil_yaw;
        camera.pitch = (self.pitch + self.recoil_pitch).clamp(-89.0, 89.0);
    }
}
