use engine::assets::asset_manager::MeshType;
use engine::{Engine, EntityId, Vec3, transform::Transform};
use std::rc::Rc;

#[derive(Clone, Copy, Debug)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

#[allow(dead_code)]
impl Aabb {
    pub fn from_center_size(center: Vec3, size: Vec3) -> Self {
        let half = size * 0.5;
        Self {
            min: center - half,
            max: center + half,
        }
    }

    pub fn intersects_point(&self, p: Vec3) -> bool {
        p.x >= self.min.x && p.x <= self.max.x &&
        p.y >= self.min.y && p.y <= self.max.y &&
        p.z >= self.min.z && p.z <= self.max.z
    }

    pub fn intersects_sphere(&self, center: Vec3, radius: f32) -> bool {
        let closest = Vec3::new(
            center.x.clamp(self.min.x, self.max.x),
            center.y.clamp(self.min.y, self.max.y),
            center.z.clamp(self.min.z, self.max.z),
        );
        (center - closest).length_squared() <= radius * radius
    }

    /// Push a sphere / cylinder out of the AABB along the shallowest penetration axis
    pub fn resolve_cylinder(&self, pos: &mut Vec3, radius: f32, height: f32) -> bool {
        // Check vertical overlap first
        let feet = pos.y;
        let head = pos.y + height;
        if head <= self.min.y || feet >= self.max.y {
            return false;
        }

        // Closest point in 2D (xz)
        let cx = pos.x.clamp(self.min.x, self.max.x);
        let cz = pos.z.clamp(self.min.z, self.max.z);

        let delta = Vec3::new(pos.x - cx, 0.0, pos.z - cz);
        let dist_sq = delta.length_squared();

        if dist_sq < radius * radius {
            let dist = dist_sq.sqrt();
            if dist > 0.0001 {
                let normal = delta / dist;
                let penetration = radius - dist;
                pos.x += normal.x * penetration;
                pos.z += normal.z * penetration;
            } else {
                // Pos is inside AABB: push out toward nearest edge
                let d_left = (pos.x - self.min.x).abs();
                let d_right = (self.max.x - pos.x).abs();
                let d_back = (pos.z - self.min.z).abs();
                let d_front = (self.max.z - pos.z).abs();

                let min_d = d_left.min(d_right).min(d_back).min(d_front);
                if min_d == d_left {
                    pos.x = self.min.x - radius;
                } else if min_d == d_right {
                    pos.x = self.max.x + radius;
                } else if min_d == d_back {
                    pos.z = self.min.z - radius;
                } else {
                    pos.z = self.max.z + radius;
                }
            }
            return true;
        }
        false
    }

    /// Slab raycast against AABB
    pub fn raycast(&self, origin: Vec3, dir: Vec3, max_dist: f32) -> Option<f32> {
        let mut tmin = 0.0_f32;
        let mut tmax = max_dist;

        for i in 0..3 {
            let (orig_comp, dir_comp, min_comp, max_comp) = match i {
                0 => (origin.x, dir.x, self.min.x, self.max.x),
                1 => (origin.y, dir.y, self.min.y, self.max.y),
                _ => (origin.z, dir.z, self.min.z, self.max.z),
            };

            if dir_comp.abs() < 1e-6 {
                if orig_comp < min_comp || orig_comp > max_comp {
                    return None;
                }
            } else {
                let inv = 1.0 / dir_comp;
                let mut t1 = (min_comp - orig_comp) * inv;
                let mut t2 = (max_comp - orig_comp) * inv;
                if t1 > t2 {
                    std::mem::swap(&mut t1, &mut t2);
                }
                tmin = tmin.max(t1);
                tmax = tmax.min(t2);
                if tmin > tmax {
                    return None;
                }
            }
        }

        if tmin >= 0.0 && tmin <= max_dist {
            Some(tmin)
        } else {
            None
        }
    }
}

pub struct JumpPad {
    pub position: Vec3,
    pub radius: f32,
    pub power: f32,
    pub _entity: EntityId,
}

pub struct Arena {
    pub colliders: Vec<Aabb>,
    pub jump_pads: Vec<JumpPad>,
    pub arena_half_size: f32,
    pub ground_y: f32,
    _entities: Vec<EntityId>,
}

impl Arena {
    pub fn new(engine: &mut Engine) -> Self {
        let ground_y = -7.0;
        let arena_size = 70.0;
        let half = arena_size * 0.5;

        // Compile custom arena floor shader
        let floor_vert = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/arena.vert"))
            .unwrap_or_else(|_| include_str!("../shaders/arena.vert").to_string());
        let floor_frag = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/arena.frag"))
            .unwrap_or_else(|_| include_str!("../shaders/arena.frag").to_string());
        let glow_vert = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/glow.vert"))
            .unwrap_or_else(|_| include_str!("../shaders/glow.vert").to_string());
        let glow_frag = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/glow.frag"))
            .unwrap_or_else(|_| include_str!("../shaders/glow.frag").to_string());

        engine.assets().create_shader("arena_floor", &floor_vert, &floor_frag);
        engine.assets().create_shader("glow", &glow_vert, &glow_frag);

        let cube_mesh = engine.assets().mesh(MeshType::Cube);

        let floor_mat = Rc::new(
            engine.assets().new_material("arena_floor").with_color(Vec3::new(0.15, 0.6, 1.0)),
        );
        let wall_mat = Rc::new(
            engine.assets().new_material("basic").with_color(Vec3::new(0.2, 0.25, 0.35)),
        );
        let pillar_mat = Rc::new(
            engine.assets().new_material("basic").with_color(Vec3::new(0.3, 0.35, 0.45)),
        );
        let trim_mat = Rc::new(
            engine.assets().new_material("glow").with_color(Vec3::new(0.0, 0.8, 1.0)),
        );
        let core_mat = Rc::new(
            engine.assets().new_material("glow").with_color(Vec3::new(0.9, 0.3, 1.0)),
        );
        let pad_mat = Rc::new(
            engine.assets().new_material("glow").with_color(Vec3::new(0.1, 1.0, 0.5)),
        );

        let mut entities = Vec::new();
        let mut colliders = Vec::new();

        // 1. Floor
        let mut floor_tr = Transform::default();
        floor_tr.position = Vec3::new(0.0, ground_y - 0.5, 0.0);
        floor_tr.scale = Vec3::new(arena_size, 1.0, arena_size);
        entities.push(engine.spawn(cube_mesh.clone(), floor_mat, floor_tr));

        // 2. Outer Perimeter Barriers (North, South, East, West)
        let wall_h = 8.0;
        let wall_thick = 2.0;

        let wall_configs = [
            (Vec3::new(0.0, ground_y + wall_h * 0.5, half), Vec3::new(arena_size + wall_thick * 2.0, wall_h, wall_thick)),
            (Vec3::new(0.0, ground_y + wall_h * 0.5, -half), Vec3::new(arena_size + wall_thick * 2.0, wall_h, wall_thick)),
            (Vec3::new(half, ground_y + wall_h * 0.5, 0.0), Vec3::new(wall_thick, wall_h, arena_size)),
            (Vec3::new(-half, ground_y + wall_h * 0.5, 0.0), Vec3::new(wall_thick, wall_h, arena_size)),
        ];

        for (pos, scale) in wall_configs {
            let mut tr = Transform::default();
            tr.position = pos;
            tr.scale = scale;
            entities.push(engine.spawn(cube_mesh.clone(), wall_mat.clone(), tr));
            colliders.push(Aabb::from_center_size(pos, scale));

            // Glowing trim along top of wall
            let mut trim_tr = Transform::default();
            trim_tr.position = Vec3::new(pos.x, pos.y + scale.y * 0.5, pos.z);
            trim_tr.scale = Vec3::new(
                if scale.x > scale.z { scale.x } else { 0.4 },
                0.3,
                if scale.z > scale.x { scale.z } else { 0.4 },
            );
            entities.push(engine.spawn(cube_mesh.clone(), trim_mat.clone(), trim_tr));
        }

        // 3. Central Spire / Hub
        let spire_pos = Vec3::new(0.0, ground_y + 4.0, 0.0);
        let spire_scale = Vec3::new(4.0, 8.0, 4.0);
        let mut tr = Transform::default();
        tr.position = spire_pos;
        tr.scale = spire_scale;
        entities.push(engine.spawn(cube_mesh.clone(), pillar_mat.clone(), tr));
        colliders.push(Aabb::from_center_size(spire_pos, spire_scale));

        // Glowing core ring
        let mut core_tr = Transform::default();
        core_tr.position = Vec3::new(0.0, ground_y + 4.5, 0.0);
        core_tr.scale = Vec3::new(4.3, 1.2, 4.3);
        entities.push(engine.spawn(cube_mesh.clone(), core_mat, core_tr));

        // 4. Strategic Cover Pillars
        let pillar_coords = [
            Vec3::new(16.0, 0.0, 16.0),
            Vec3::new(-16.0, 0.0, 16.0),
            Vec3::new(16.0, 0.0, -16.0),
            Vec3::new(-16.0, 0.0, -16.0),
            Vec3::new(24.0, 0.0, 0.0),
            Vec3::new(-24.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 24.0),
            Vec3::new(0.0, 0.0, -24.0),
        ];

        for &coord in &pillar_coords {
            let p_h = 6.0;
            let p_pos = Vec3::new(coord.x, ground_y + p_h * 0.5, coord.z);
            let p_scale = Vec3::new(2.4, p_h, 2.4);

            let mut tr = Transform::default();
            tr.position = p_pos;
            tr.scale = p_scale;
            entities.push(engine.spawn(cube_mesh.clone(), pillar_mat.clone(), tr));
            colliders.push(Aabb::from_center_size(p_pos, p_scale));

            // Glowing accent band
            let mut tr_band = Transform::default();
            tr_band.position = Vec3::new(coord.x, ground_y + 3.0, coord.z);
            tr_band.scale = Vec3::new(2.5, 0.3, 2.5);
            entities.push(engine.spawn(cube_mesh.clone(), trim_mat.clone(), tr_band));
        }

        // 5. Tactical Low Cover Barricades
        let barricade_coords = [
            (Vec3::new(8.0, ground_y + 1.25, 0.0), Vec3::new(1.2, 2.5, 6.0)),
            (Vec3::new(-8.0, ground_y + 1.25, 0.0), Vec3::new(1.2, 2.5, 6.0)),
            (Vec3::new(0.0, ground_y + 1.25, 8.0), Vec3::new(6.0, 2.5, 1.2)),
            (Vec3::new(0.0, ground_y + 1.25, -8.0), Vec3::new(6.0, 2.5, 1.2)),
            (Vec3::new(18.0, ground_y + 1.5, 8.0), Vec3::new(4.0, 3.0, 1.2)),
            (Vec3::new(-18.0, ground_y + 1.5, -8.0), Vec3::new(4.0, 3.0, 1.2)),
        ];

        for (b_pos, b_scale) in barricade_coords {
            let mut tr = Transform::default();
            tr.position = b_pos;
            tr.scale = b_scale;
            entities.push(engine.spawn(cube_mesh.clone(), wall_mat.clone(), tr));
            colliders.push(Aabb::from_center_size(b_pos, b_scale));
        }

        // 6. Jump Pads
        let pad_positions = [
            Vec3::new(0.0, ground_y + 0.15, -20.0),
            Vec3::new(0.0, ground_y + 0.15, 20.0),
            Vec3::new(-20.0, ground_y + 0.15, 0.0),
            Vec3::new(20.0, ground_y + 0.15, 0.0),
        ];

        let mut jump_pads = Vec::new();
        for &pos in &pad_positions {
            let mut tr = Transform::default();
            tr.position = pos;
            tr.scale = Vec3::new(2.8, 0.3, 2.8);
            let entity = engine.spawn(cube_mesh.clone(), pad_mat.clone(), tr);
            entities.push(entity);

            jump_pads.push(JumpPad {
                position: pos,
                radius: 2.0,
                power: 18.0,
                _entity: entity,
            });
        }

        Self {
            colliders,
            jump_pads,
            arena_half_size: half - 1.5,
            ground_y,
            _entities: entities,
        }
    }

    pub fn resolve_player_collision(&self, pos: &mut Vec3, radius: f32, height: f32) {
        // Outer boundaries
        pos.x = pos.x.clamp(-self.arena_half_size + radius, self.arena_half_size - radius);
        pos.z = pos.z.clamp(-self.arena_half_size + radius, self.arena_half_size - radius);

        // Obstacles
        for c in &self.colliders {
            c.resolve_cylinder(pos, radius, height);
        }
    }

    pub fn check_jump_pads(&self, pos: Vec3) -> Option<f32> {
        for pad in &self.jump_pads {
            let dx = pos.x - pad.position.x;
            let dz = pos.z - pad.position.z;
            if dx * dx + dz * dz <= pad.radius * pad.radius && (pos.y - pad.position.y).abs() < 1.5 {
                return Some(pad.power);
            }
        }
        None
    }

    pub fn raycast(&self, origin: Vec3, dir: Vec3, max_dist: f32) -> Option<f32> {
        let mut closest = max_dist;
        let mut hit = false;

        for c in &self.colliders {
            if let Some(t) = c.raycast(origin, dir, closest) {
                closest = t;
                hit = true;
            }
        }

        // Also check floor
        if dir.y.abs() > 1e-5 {
            let t_ground = (self.ground_y - origin.y) / dir.y;
            if t_ground > 0.0 && t_ground < closest {
                let hit_x = origin.x + dir.x * t_ground;
                let hit_z = origin.z + dir.z * t_ground;
                if hit_x.abs() <= self.arena_half_size && hit_z.abs() <= self.arena_half_size {
                    closest = t_ground;
                    hit = true;
                }
            }
        }

        if hit {
            Some(closest)
        } else {
            None
        }
    }
}
